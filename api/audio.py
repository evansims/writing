import hashlib
import os
import re

import frontmatter
from elevenlabs.client import ElevenLabs
from sanic import Blueprint, Request, response
from sanic.exceptions import NotFound, ServerError
from sanic.response import JSONResponse
from sanic.response import json as json_response

from ._filesystem import cached_file_exists, cached_file_read, get_content_dir
from ._types import Page
from ._validation import is_valid_path, is_valid_slug, safe_path
from .content import _page

# Print the loaded environment variables for debugging
API_KEY = os.getenv("EVANSIMS_ELEVENLABS_API_KEY")
VOICE_ID = os.getenv("EVANSIMS_ELEVENLABS_VOICE_ID") or "bIHbv24MWmeRgasZH58o"
MODEL_ID = os.getenv("EVANSIMS_ELEVENLABS_MODEL_ID") or "eleven_multilingual_v2"

# Initialize Eleven Labs client
client = ElevenLabs(api_key=API_KEY)

# Create the audio blueprint
audio_bp = Blueprint("audio_routes", url_prefix="/api/audio")


def get_audio_dir(content_path: str) -> str:
    """Return the directory for storing audio files next to content."""
    content_dir = os.path.dirname(content_path)
    audio_dir = os.path.join(content_dir, "audio")
    os.makedirs(audio_dir, exist_ok=True)
    return audio_dir


def split_content_into_chunks(
    content: str, title: str = None, description: str = None
) -> list[dict]:
    """Split markdown content into logical chunks based on h2 headings.

    Returns a list of dictionaries with 'id', 'text', and 'checksum' for each chunk.
    Each chunk contains all content from one h2 heading to the next h2 heading.
    The first chunk contains all content before the first h2 heading.
    """
    # Remove frontmatter from the content
    try:
        post = frontmatter.loads(content)
        content = post.content

        # If not provided as parameters, try to get title and description from frontmatter
        if title is None and post.get("title"):
            title = post.get("title")
        if description is None and post.get("description"):
            description = post.get("description")
    except Exception:
        # If parsing fails, assume the content has no frontmatter
        pass

    # Clean the content for better TTS processing
    content = re.sub(r"\n\n+", "\n\n", content)  # Normalize line breaks

    # Split content by h2 headings
    # Match ## Heading patterns (with optional spaces after ##)
    h2_pattern = r"(?m)^##\s+(.+)$"

    # Find all h2 headings and their positions
    headings = [
        (m.group(0), m.start(), m.group(1).strip())
        for m in re.finditer(h2_pattern, content)
    ]

    chunks = []

    # Prepare intro text with title, description and attribution if available
    intro_prefix = ""
    if title:
        intro_prefix += f"{title}. "
    if description and description.lower() != "none":
        intro_prefix += f"{description} "
    intro_prefix += (
        "by Evan Sims. . . . . "  # Multiple periods to create a longer pause
    )

    if headings:
        # First chunk: content before the first heading
        if headings[0][1] > 0:
            intro_text = intro_prefix + content[: headings[0][1]].strip()
            if intro_text:
                clean_intro = clean_text_for_tts(intro_text)
                if len(clean_intro) >= 3:  # Skip if too short
                    checksum = hashlib.md5(clean_intro.encode("utf-8")).hexdigest()
                    chunks.append(
                        {
                            "id": "intro",
                            "text": clean_intro,
                            "checksum": checksum,
                        }
                    )

        # Process each heading and its content
        for i, (heading_text, start_pos, heading_title) in enumerate(headings):
            # Find the end position (next heading or end of content)
            end_pos = headings[i + 1][1] if i < len(headings) - 1 else len(content)

            # Extract section content including the heading
            section_text = content[start_pos:end_pos].strip()

            if section_text:
                # Clean the text for TTS
                clean_section = clean_text_for_tts(section_text)

                if len(clean_section) >= 3:  # Skip if too short
                    section_id = f"section_{i}"
                    checksum = hashlib.md5(clean_section.encode("utf-8")).hexdigest()
                    chunks.append(
                        {
                            "id": section_id,
                            "text": clean_section,
                            "title": heading_title,  # Store the heading title
                            "checksum": checksum,
                        }
                    )
    else:
        # No headings found, process the entire content as one chunk
        if content.strip():
            full_text = intro_prefix + content.strip()
            clean_content = clean_text_for_tts(full_text)
            if len(clean_content) >= 3:
                checksum = hashlib.md5(clean_content.encode("utf-8")).hexdigest()
                chunks.append(
                    {
                        "id": "full_content",
                        "text": clean_content,
                        "checksum": checksum,
                    }
                )

    return chunks


def clean_text_for_tts(text: str) -> str:
    """Clean text for TTS processing by removing markdown formatting."""
    # Remove markdown headers but keep the text
    clean_text = re.sub(r"^#+\s+", "", text, flags=re.MULTILINE)

    # Remove markdown formatting
    clean_text = re.sub(r"\*\*|\*|__|\^", "", clean_text)

    # Convert links to just text
    clean_text = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", clean_text)

    # Remove code blocks
    clean_text = re.sub(r"```[^`]*```", " ", clean_text)

    # Remove inline code
    clean_text = re.sub(r"`([^`]+)`", r"\1", clean_text)

    # Remove excessive spaces
    clean_text = re.sub(r"\s+", " ", clean_text).strip()

    return clean_text


def get_audio_path(audio_dir: str, checksum: str) -> str:
    """Return the path to the audio file for a given content checksum."""
    return os.path.join(audio_dir, f"{checksum}.mp3")


async def get_or_generate_audio(chunk_text: str, audio_path: str) -> bytes:
    """Get existing audio file or generate new one using Eleven Labs."""
    if os.path.exists(audio_path):
        # Return cached audio
        with open(audio_path, "rb") as f:
            audio_data = f.read()
            # Check if file is empty
            if not audio_data or len(audio_data) == 0:
                print(
                    f"Warning: Found empty audio file at {audio_path}, regenerating..."
                )
                # Fall through to regenerate
            else:
                return audio_data

    # Generate new audio
    try:
        print(
            f"Generating audio for text: '{chunk_text[:50]}...' using Eleven Labs API"
        )
        print(f"Using voice_id={VOICE_ID}, model_id={MODEL_ID}")

        # Use the text_to_speech.convert method from the client
        audio_data = client.text_to_speech.convert(
            text=chunk_text,
            voice_id=VOICE_ID,
            model_id=MODEL_ID,
            output_format="mp3_44100_128",
        )

        # Handle if audio_data is a generator
        if hasattr(audio_data, "__iter__") and not isinstance(
            audio_data, (bytes, bytearray)
        ):
            print("Converting generator to bytes")
            audio_bytes = b"".join(chunk for chunk in audio_data)
        else:
            audio_bytes = audio_data

        # Check if we got data back
        if not audio_bytes or len(audio_bytes) == 0:
            print("Error: Received empty audio data from Eleven Labs API")
            raise ValueError("Received empty audio data from Eleven Labs API")

        print(f"Successfully generated {len(audio_bytes)} bytes of audio data")

        # Cache the generated audio
        try:
            # Ensure directory exists
            os.makedirs(os.path.dirname(audio_path), exist_ok=True)

            with open(audio_path, "wb") as f:
                f.write(audio_bytes)

            print(f"Saved audio file to {audio_path}")
        except Exception as file_error:
            print(f"Error saving audio file: {str(file_error)}")
            # Continue even if saving fails

        return audio_bytes
    except Exception as e:
        print(f"Error generating audio: {str(e)}")
        import traceback

        traceback.print_exc()

        # Remove any empty file that might have been created
        if os.path.exists(audio_path):
            try:
                file_size = os.path.getsize(audio_path)
                if file_size == 0:
                    print(f"Removing empty audio file: {audio_path}")
                    os.remove(audio_path)
            except Exception as cleanup_error:
                print(f"Error cleaning up empty file: {str(cleanup_error)}")

        raise ServerError(f"Failed to generate audio: {str(e)}")


@audio_bp.get("/")
async def audio_health_check(request: Request) -> JSONResponse:
    """Return API health status and configuration information."""
    try:
        return json_response(
            {
                "status": "OK" if API_KEY else "WARNING",
                "api_key_valid": bool(API_KEY),
                "voice_id": VOICE_ID or "default",
                "model_id": MODEL_ID or "default",
                "message": "Audio API is running",
            }
        )
    except Exception:
        import traceback

        return json_response(
            {
                "status": "ERROR",
                "message": "Audio API encountered an error during health check",
                "error": traceback.format_exc(),
            },
            status=500,
        )


@audio_bp.get("/<slug:path>/metadata")
async def get_audio_metadata(request: Request, slug: str) -> JSONResponse:
    """Return metadata about available audio chunks for a page."""
    if not is_valid_slug(slug):
        raise NotFound("Invalid slug")

    try:
        # Get the content page
        f = safe_path(f"{slug}/{slug}.md")
        page: Page = await _page(f, slug)

        # Get content and split into chunks
        content = cached_file_read(page.path)
        chunks = split_content_into_chunks(content, page.title, page.description)

        # Check which chunks have audio files
        audio_dir = get_audio_dir(page.path)
        for chunk in chunks:
            audio_path = get_audio_path(audio_dir, chunk["checksum"])
            chunk["has_audio"] = os.path.exists(audio_path)

        return json_response(
            {
                "page": {
                    "slug": page.slug,
                    "title": page.title,
                },
                "chunks": chunks,
            }
        )
    except Exception:
        raise NotFound("Failed to get audio metadata")


@audio_bp.get("/<slug:path>/<chunk_id:str>")
async def get_chunk_audio(
    request: Request, slug: str, chunk_id: str
) -> response.HTTPResponse:
    """Return audio for a specific chunk of content."""
    if not is_valid_slug(slug):
        raise NotFound("Invalid slug")

    try:
        # Try to see if this is actually a folder/page request without a chunk ID
        file_path = f"{slug}/{chunk_id}/{chunk_id}.md"
        full_path = safe_path(file_path)

        print("DEBUG: Checking if this is a nested path request without chunk ID")
        print(f"DEBUG: Testing file path: {file_path}")
        print(f"DEBUG: Full path: {full_path}")
        print(f"DEBUG: File exists: {cached_file_exists(full_path)}")

        if cached_file_exists(full_path):
            # This is actually a page request, not a chunk request
            # Redirect to the nested page endpoint
            return await get_nested_page_audio(request, slug, chunk_id)

        # Check for actual chunk within a normal page
        if "/" in slug:
            # For nested paths like "mindset/downtime-as-self-care"
            parts = slug.split("/")
            folder = "/".join(parts[:-1])  # "mindset"
            page_name = parts[-1]  # "downtime-as-self-care"

            file_path = f"{folder}/{page_name}/{page_name}.md"
            print(f"DEBUG: Looking for content at path: {file_path}")

            full_path = safe_path(file_path)
            if not cached_file_exists(full_path):
                raise NotFound(f"Content file not found at {file_path}")

            page: Page = await _page(full_path, page_name)
        else:
            # For top-level content
            f = safe_path(f"{slug}/{slug}.md")
            page: Page = await _page(f, slug)

        # Get content and split into chunks
        content = cached_file_read(page.path)
        chunks = split_content_into_chunks(content, page.title, page.description)

        # Find the requested chunk
        chunk = next((c for c in chunks if c["id"] == chunk_id), None)
        if not chunk:
            raise NotFound(f"Chunk {chunk_id} not found")

        # Generate audio path
        audio_dir = get_audio_dir(page.path)
        audio_path = get_audio_path(audio_dir, chunk["checksum"])

        # Check if we should force regeneration
        force_regenerate = request.args.get("regenerate", False)
        if force_regenerate and os.path.exists(audio_path):
            os.remove(audio_path)

        # Use default voice (Will)
        audio_data = await get_or_generate_audio(chunk["text"], audio_path)
        return response.raw(audio_data, content_type="audio/mpeg")
    except Exception as e:
        import traceback

        print(f"Error in get_chunk_audio: {str(e)}")
        print(traceback.format_exc())

        raise NotFound(f"Failed to get audio: {str(e)}")


@audio_bp.get("/<slug:path>")
async def get_page_audio(request: Request, slug: str) -> JSONResponse:
    """Return audio metadata for a page or generate all audio."""
    if not is_valid_slug(slug):
        raise NotFound("Invalid slug")

    generate_all = request.args.get("generate_all", False)

    try:
        # Check if the slug contains a path separator and parse accordingly
        if "/" in slug:
            # For nested paths like "mindset/downtime-as-self-care"
            parts = slug.split("/")
            folder = "/".join(parts[:-1])  # "mindset"
            page_name = parts[-1]  # "downtime-as-self-care"

            print(f"DEBUG: Processing nested path: folder={folder}, page={page_name}")

            file_path = f"{folder}/{page_name}/{page_name}.md"
            full_path = safe_path(file_path)

            if not cached_file_exists(full_path):
                raise NotFound(f"Content file not found at {file_path}")

            page: Page = await _page(full_path, page_name)
        else:
            # For top-level content
            f = safe_path(f"{slug}/{slug}.md")
            page: Page = await _page(f, slug)

        # Get content and split into chunks
        content = cached_file_read(page.path)
        chunks = split_content_into_chunks(content, page.title, page.description)

        audio_dir = get_audio_dir(page.path)

        if generate_all:
            # Generate audio for all chunks
            for chunk in chunks:
                audio_path = get_audio_path(audio_dir, chunk["checksum"])
                await get_or_generate_audio(chunk["text"], audio_path)
                chunk["has_audio"] = True
        else:
            # Just check which chunks have audio
            for chunk in chunks:
                audio_path = get_audio_path(audio_dir, chunk["checksum"])
                chunk["has_audio"] = os.path.exists(audio_path)

        return json_response(
            {
                "page": {
                    "slug": page.slug,
                    "title": page.title,
                },
                "chunks": chunks,
            }
        )
    except Exception as e:
        import traceback

        print(f"Error in get_page_audio: {str(e)}")
        print(traceback.format_exc())

        raise NotFound(f"Failed to get audio metadata: {str(e)}")


@audio_bp.get("/<folder:path>/<page:slug>/<chunk_id:str>")
async def get_nested_chunk_audio(
    request: Request, folder: str, page: str, chunk_id: str
) -> response.HTTPResponse:
    """Return audio for a specific chunk of content in a nested folder."""
    if not is_valid_path(folder) or not is_valid_slug(page):
        raise NotFound("Invalid path or slug")

    try:
        # Get the content page
        f = safe_path(f"{folder}/{page}/{page}.md")
        page_obj: Page = await _page(f, page)

        # Get content and split into chunks
        content = cached_file_read(page_obj.path)
        chunks = split_content_into_chunks(
            content, page_obj.title, page_obj.description
        )

        # Find the requested chunk
        chunk = next((c for c in chunks if c["id"] == chunk_id), None)
        if not chunk:
            raise NotFound(f"Chunk {chunk_id} not found")

        # Generate audio path
        audio_dir = get_audio_dir(page_obj.path)
        audio_path = get_audio_path(audio_dir, chunk["checksum"])

        # Check if we should force regeneration
        force_regenerate = request.args.get("regenerate", False)
        if force_regenerate and os.path.exists(audio_path):
            os.remove(audio_path)

        # Use default voice (Will)
        audio_data = await get_or_generate_audio(chunk["text"], audio_path)
        return response.raw(audio_data, content_type="audio/mpeg")
    except Exception as e:
        raise NotFound(f"Failed to get audio: {str(e)}")


@audio_bp.get("/<folder:path>/<page:slug>")
async def get_nested_page_audio(
    request: Request, folder: str, page: str
) -> JSONResponse:
    """Return audio metadata for a nested page or generate all audio."""
    if not is_valid_path(folder) or not is_valid_slug(page):
        raise NotFound("Invalid path or slug")

    generate_all = request.args.get("generate_all", False)

    try:
        # Debug info for path troubleshooting
        content_dir = get_content_dir()
        file_path = f"{folder}/{page}/{page}.md"
        full_path = safe_path(file_path)
        file_exists = cached_file_exists(full_path)

        # Log detailed path information for debugging
        print("AUDIO API DEBUG - Nested Content request details:")
        print(f"  Request URL: {request.url}")
        print(f"  Folder: {folder}")
        print(f"  Page: {page}")
        print(f"  Content directory: {content_dir}")
        print(f"  Relative file path: {file_path}")
        print(f"  Computed full path: {full_path}")
        print(f"  File exists: {file_exists}")

        # If file doesn't exist, try to help diagnose
        if not file_exists:
            parent_dir = os.path.dirname(full_path)
            print(f"  Parent directory: {parent_dir}")
            print(f"  Parent exists: {os.path.exists(parent_dir)}")

            if os.path.exists(parent_dir):
                print(f"  Files in parent dir: {os.listdir(parent_dir)}")

            # Try alternative path patterns
            alt_path1 = safe_path(f"{folder}/{page}.md")
            alt_path2 = safe_path(f"{page}/{page}.md")
            print(f"  Alternative path 1: {alt_path1}")
            print(f"  Alternative path 1 exists: {cached_file_exists(alt_path1)}")
            print(f"  Alternative path 2: {alt_path2}")
            print(f"  Alternative path 2 exists: {cached_file_exists(alt_path2)}")

        if not file_exists:
            raise NotFound(f"Content file not found: {file_path}")

        # Get the content page
        page_obj: Page = await _page(full_path, page)

        # Get content and split into chunks
        content = cached_file_read(page_obj.path)
        chunks = split_content_into_chunks(
            content, page_obj.title, page_obj.description
        )

        audio_dir = get_audio_dir(page_obj.path)

        if generate_all:
            # Generate audio for all chunks
            for chunk in chunks:
                audio_path = get_audio_path(audio_dir, chunk["checksum"])
                await get_or_generate_audio(chunk["text"], audio_path)
                chunk["has_audio"] = True
        else:
            # Just check which chunks have audio
            for chunk in chunks:
                audio_path = get_audio_path(audio_dir, chunk["checksum"])
                chunk["has_audio"] = os.path.exists(audio_path)

        return json_response(
            {
                "page": {
                    "slug": page_obj.slug,
                    "title": page_obj.title,
                },
                "chunks": chunks,
            }
        )
    except Exception as e:
        import traceback

        print(f"Audio API Error: {str(e)}")
        print(traceback.format_exc())

        raise NotFound(f"Failed to get audio metadata: {str(e)}")


@audio_bp.get("/debug/voices")
async def get_voices_endpoint(request: Request) -> JSONResponse:
    """Debug endpoint to check the Eleven Labs API and list available voices."""
    try:
        available_voices = client.voices.get_all()
        voice_list = []

        # Add debug output to check the structure
        print(f"Type of available_voices: {type(available_voices)}")
        if hasattr(available_voices, "voices"):
            voices_to_iterate = available_voices.voices
        else:
            # Handle case where voices might be directly in the response
            voices_to_iterate = available_voices

        print(
            f"First voice item type: {type(voices_to_iterate[0]) if voices_to_iterate else 'No voices'}"
        )

        # Handle both object and tuple structures
        for voice in voices_to_iterate:
            # If voice is a tuple, it might be (name, voice_id, ...) format
            if isinstance(voice, tuple):
                # Extract data based on position if it's a tuple
                voice_data = {
                    "name": voice[0] if len(voice) > 0 else "Unknown",
                    "voice_id": voice[1] if len(voice) > 1 else "Unknown",
                    "category": voice[2] if len(voice) > 2 else "unknown",
                    "description": voice[3] if len(voice) > 3 else "",
                    "preview_url": voice[4] if len(voice) > 4 else "",
                }
            else:
                # If it's an object with attributes
                voice_data = {
                    "name": getattr(voice, "name", "Unknown"),
                    "voice_id": getattr(voice, "voice_id", "Unknown"),
                    "category": getattr(voice, "category", "unknown"),
                    "description": getattr(voice, "description", ""),
                    "preview_url": getattr(voice, "preview_url", ""),
                }

            voice_list.append(voice_data)

        return json_response(
            {
                "status": "success",
                "voice_count": len(voice_list),
                "voices": voice_list,
            }
        )
    except Exception as e:
        print(f"Error getting voices: {str(e)}")
        import traceback

        traceback.print_exc()

        return json_response(
            {
                "status": "error",
                "error": str(e),
                "api_key_valid": False,
                "message": "Failed to get voices from Eleven Labs API. Check your API key and connection.",
            },
            status=500,
        )


@audio_bp.get("/debug/test")
async def test_audio_generation(request: Request) -> JSONResponse:
    """Test audio generation with a simple phrase."""

    try:
        test_text = "This is a test of the Eleven Labs text to speech API."
        test_checksum = hashlib.md5(test_text.encode("utf-8")).hexdigest()
        test_dir = os.path.join(get_content_dir(), "audio_tests")
        os.makedirs(test_dir, exist_ok=True)
        test_path = os.path.join(test_dir, f"{test_checksum}.mp3")

        # Force regeneration
        if os.path.exists(test_path):
            os.remove(test_path)

        # Generate audio
        print(f"Testing audio generation with text: '{test_text}'")
        audio_data = client.text_to_speech.convert(
            text=test_text,
            voice_id=VOICE_ID,
            model_id=MODEL_ID,
            output_format="mp3_44100_128",
        )

        # Handle if audio_data is a generator
        if hasattr(audio_data, "__iter__") and not isinstance(
            audio_data, (bytes, bytearray)
        ):
            print("Converting generator to bytes")
            audio_bytes = b"".join(chunk for chunk in audio_data)
        else:
            audio_bytes = audio_data

        # Verify we have valid audio data
        if not audio_bytes or len(audio_bytes) == 0:
            raise ValueError("Generated audio data is empty")

        # Save test audio
        with open(test_path, "wb") as f:
            f.write(audio_bytes)

        file_size = os.path.getsize(test_path)

        print(f"Successfully saved test audio ({file_size} bytes)")

        return json_response(
            {
                "status": "success",
                "message": "Successfully generated test audio",
                "file_size": file_size,
                "file_path": test_path,
                "text": test_text,
                "voice_id": VOICE_ID,
                "model_id": MODEL_ID,
            }
        )
    except Exception as e:
        print(f"Error in test audio generation: {str(e)}")
        import traceback

        traceback.print_exc()

        return json_response(
            {
                "status": "error",
                "error": str(e),
                "message": "Failed to generate test audio. See server logs for details.",
            },
            status=500,
        )
