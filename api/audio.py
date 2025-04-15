import hashlib
import os
import re
from collections.abc import AsyncGenerator, Iterator

import dotenv
from elevenlabs.client import ElevenLabs
from fastapi import FastAPI
from fastapi.responses import JSONResponse, StreamingResponse

from api._content import _page
from api._filesystem import cached_file_exists
from api._types import Page
from api._validation import is_valid_path, is_valid_slug, safe_path

app = FastAPI()

dotenv.load_dotenv()

# Print the loaded environment variables for debugging
API_KEY = os.getenv("EVANSIMS_ELEVENLABS_API_KEY")
VOICE_ID = os.getenv("EVANSIMS_ELEVENLABS_VOICE_ID") or "bIHbv24MWmeRgasZH58o"
MODEL_ID = os.getenv("EVANSIMS_ELEVENLABS_MODEL_ID") or "eleven_multilingual_v2"

# Initialize Eleven Labs client
client = ElevenLabs(api_key=API_KEY)


def get_audio_dir(content_path: str) -> str:
    """Return the directory for storing audio files next to content."""
    content_dir = os.path.dirname(content_path)
    audio_dir = os.path.join(content_dir, "audio")
    os.makedirs(audio_dir, exist_ok=True)
    return audio_dir


async def split_content_into_chunks(
    content: str, title: str = None, description: str = None, page_path: str = None
) -> list[dict]:
    """Split markdown content into logical chunks based on h2 headings.

    Returns a list of dictionaries with 'id', 'text', and 'checksum' for each chunk.
    Each chunk contains all content from one h2 heading to the next h2 heading.
    The first chunk contains all content before the first h2 heading.

    If page_path is provided, will use _page function to parse frontmatter. Otherwise,
    expects raw content with optional title and description parameters.
    """
    # If a page path is provided, use the _page function to get the Page object
    if page_path:
        try:
            # Extract slug from the filename
            slug = os.path.basename(page_path).replace(".md", "")
            page = _page(page_path, slug)
            content = page.body
            if title is None:
                title = page.title
            if description is None and page.description:
                description = page.description
        except Exception as e:
            print(f"Error loading page from path {page_path}: {e}")
    # Clean the content for better TTS processing
    content = re.sub(r"\n\n+", "\n\n", content)  # Normalize line breaks

    # Split content by h2 headings
    # Match ## Heading patterns (with optional spaces after ##)
    h2_pattern = r"(?m)^##\s+(.+)$"

    # Find all h2 headings and their positions
    headings = [(m.group(0), m.start(), m.group(1).strip()) for m in re.finditer(h2_pattern, content)]

    chunks = []

    # Prepare intro text with title, description and attribution if available
    intro_prefix = ""
    if title:
        intro_prefix += f"{title}. "
    if description and description.lower() != "none":
        intro_prefix += f"{description} "
    intro_prefix += "by Evan Sims. . . . . "  # Multiple periods to create a longer pause

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
        for i, (_, start_pos, heading_title) in enumerate(headings):
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


def get_full_audio_path(audio_dir: str, page_slug: str) -> str:
    """Return the path to the full concatenated audio file for a page."""
    return os.path.join(audio_dir, f"{page_slug}_full.mp3")


async def get_or_generate_audio(chunk_text: str, audio_path: str) -> AsyncGenerator[bytes, None]:
    """Get existing audio file or generate new one using Eleven Labs."""
    if os.path.exists(audio_path):
        # Return cached audio
        with open(audio_path, "rb") as f:
            audio_data = f.read()
            # Check if file is empty
            if not audio_data or len(audio_data) == 0:
                print(f"Warning: Found empty audio file at {audio_path}, regenerating...")
                # Fall through to regenerate
            else:
                yield audio_data

    # Generate new audio
    try:
        print(f"Generating audio for text: '{chunk_text[:50]}...' using Eleven Labs API")
        print(f"Using voice_id={VOICE_ID}, model_id={MODEL_ID}")

        # Use the text_to_speech.convert method from the client
        audio_data = client.text_to_speech.convert(
            text=chunk_text,
            voice_id=VOICE_ID,
            model_id=MODEL_ID,
            output_format="mp3_44100_128",
        )

        # Handle if audio_data is a generator
        if hasattr(audio_data, "__iter__") and not isinstance(audio_data, bytes | bytearray):
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

        yield audio_bytes
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

        raise Exception(f"Failed to generate audio: {str(e)}") from e


async def generate_or_get_full_audio(page_path: str, page_slug: str) -> tuple[str, int]:
    """Generate or get a full audio file from all chunks.

    Returns:
        tuple[str, int]: Path to the full audio file and its size in bytes

    """
    try:
        # Get audio directory
        audio_dir = get_audio_dir(page_path)
        full_audio_path = get_full_audio_path(audio_dir, page_slug)

        # Check if full audio already exists
        if os.path.exists(full_audio_path):
            return full_audio_path, os.path.getsize(full_audio_path)

        # Get all chunks and generate audio for each one if needed
        chunks = await split_content_into_chunks(content="", page_path=page_path)

        # Check if all chunks have audio files
        all_chunks_ready = True
        audio_paths = []

        for chunk in chunks:
            chunk_audio_path = get_audio_path(audio_dir, chunk.get("checksum", ""))
            if not os.path.exists(chunk_audio_path):
                # Generate missing audio
                print(f"Generating missing audio for chunk {chunk['id']}")
                audio_data = b""
                async for data in get_or_generate_audio(chunk["text"], chunk_audio_path):
                    audio_data += data
                all_chunks_ready = bool(audio_data) and all_chunks_ready

            if os.path.exists(chunk_audio_path):
                audio_paths.append(chunk_audio_path)

        if not all_chunks_ready or not audio_paths:
            raise Exception("Failed to generate all required audio chunks")

        # Concatenate audio files
        with open(full_audio_path, "wb") as outfile:
            # For MP3 files, we need to be careful about concatenation
            # This simple approach works for MP3s from the same source with the same settings
            for audio_path in audio_paths:
                with open(audio_path, "rb") as infile:
                    outfile.write(infile.read())

        return full_audio_path, os.path.getsize(full_audio_path)

    except Exception as e:
        print(f"Error generating full audio: {str(e)}")
        import traceback

        traceback.print_exc()
        raise Exception(f"Failed to generate full audio: {str(e)}") from e


@app.get("/api/audio/")
async def audio_health_check() -> JSONResponse:
    """Return API health status and configuration information."""
    try:
        return JSONResponse(
            {
                "status": "OK" if API_KEY else "WARNING",
                "api_key_valid": bool(API_KEY),
                "voice_id": VOICE_ID or "default",
                "model_id": MODEL_ID or "default",
                "message": "Audio API is running",
            }
        )
    except Exception as e:
        raise Exception(f"Failed to check audio API health: {str(e)}") from e


@app.get("/api/audio/<slug>/metadata")
async def get_audio_metadata(slug: str) -> JSONResponse:
    """Return metadata about available audio chunks for a page."""
    if not is_valid_slug(slug):
        raise Exception("Invalid slug")

    try:
        # Get the content page
        f = safe_path(f"{slug}/{slug}.md")
        page: Page = _page(f, slug)

        # Get content and split into chunks using page path directly
        chunks = await split_content_into_chunks(
            content="",  # Not needed when using page_path
            title=page.title,
            description=page.description,
            page_path=page.path,
        )

        # Check which chunks have audio files
        audio_dir = get_audio_dir(page.path)
        for chunk in chunks:
            audio_path = get_audio_path(audio_dir, chunk["checksum"])
            chunk["has_audio"] = os.path.exists(audio_path)

        return JSONResponse(
            {
                "page": {
                    "slug": page.slug,
                    "title": page.title,
                },
                "chunks": chunks,
            }
        )
    except Exception as e:
        raise Exception(f"Failed to get audio metadata: {str(e)}") from e


@app.get("/api/audio/<slug>/<chunk_id>")
async def get_chunk_audio(slug: str, chunk_id: str) -> StreamingResponse:
    """Return audio for a specific chunk of content."""
    if not is_valid_slug(slug):
        raise Exception("Invalid slug")

    try:
        # Try to see if this is actually a folder/page request without a chunk ID
        file_path = f"{slug}/{chunk_id}/{chunk_id}.md"
        full_path = safe_path(file_path)

        if cached_file_exists(full_path):
            # This is actually a page request, not a chunk request
            # Redirect to the nested page endpoint
            return await get_nested_page_audio(slug, chunk_id)

        # Check for actual chunk within a normal page
        if "/" in slug:
            # For nested paths like "mindset/downtime-as-self-care"
            parts = slug.split("/")
            folder = "/".join(parts[:-1])  # "mindset"
            page_name = parts[-1]  # "downtime-as-self-care"

            file_path = f"{folder}/{page_name}/{page_name}.md"

            full_path = safe_path(file_path)
            if not cached_file_exists(full_path):
                raise Exception(f"Content file not found at {file_path}") from None

            page: Page = _page(full_path, page_name)
        else:
            # For top-level content
            f = safe_path(f"{slug}/{slug}.md")
            page: Page = _page(f, slug)

        # Get content and split into chunks using page path directly
        chunks = await split_content_into_chunks(
            content="",  # Not needed when using page_path
            title=page.title,
            description=page.description,
            page_path=page.path,
        )

        # Find the requested chunk
        chunk = next((c for c in chunks if c["id"] == chunk_id), None)
        if not chunk:
            raise Exception(f"Chunk {chunk_id} not found") from None

        # Generate audio path
        audio_dir = get_audio_dir(page.path)
        audio_path = get_audio_path(audio_dir, chunk["checksum"])

        # Use default voice (Will)
        audio_data = await get_or_generate_audio(chunk["text"], audio_path)

        return StreamingResponse(audio_data, media_type="audio/mpeg")

    except Exception as e:
        import traceback

        print(f"Error in get_chunk_audio: {str(e)}")
        print(traceback.format_exc())

        raise Exception(f"Failed to get audio: {str(e)}") from e


@app.get("/api/audio/{slug}", response_model=None)
async def get_page_audio(
    slug: str, generate_all: bool | None = None, format: str | None = None
) -> JSONResponse | StreamingResponse:
    """Return audio metadata for a page or generate all audio.

    Args:
        slug: The slug of the page
        generate_all: Whether to generate all audio chunks
        format: If 'mp3', return a full audio file instead of metadata
    """
    # Check if the slug actually contains multiple path parts
    if "/" in slug and format == "mp3":
        # For URLs like /api/audio/mindset/downtime-as-self-care?format=mp3
        parts = slug.split("/", 1)  # Split only on the first slash
        return await get_nested_page_audio(parts[0], parts[1], format=format)

    if not is_valid_slug(slug):
        raise Exception("Invalid slug")

    generate_all = generate_all or False
    format = format or "json"

    try:
        # Check if the slug contains a path separator and parse accordingly
        if "/" in slug:
            # For nested paths like "mindset/downtime-as-self-care"
            parts = slug.split("/")
            folder = "/".join(parts[:-1])  # "mindset"
            page_name = parts[-1]  # "downtime-as-self-care"

            file_path = f"{folder}/{page_name}/{page_name}.md"
            full_path = safe_path(file_path)

            if not cached_file_exists(full_path):
                raise Exception(f"Content file not found at {file_path}")

            page: Page = _page(full_path, page_name)
        else:
            # For top-level content
            f = safe_path(f"{slug}/{slug}.md")
            page: Page = _page(f, slug)

        # If MP3 format requested, return full audio file
        if format.lower() == "mp3":
            # Generate or get full audio file
            audio_path, audio_size = await generate_or_get_full_audio(page.path, page.slug)

            # Return streaming response with audio file
            def iterfile() -> Iterator[bytes]:
                with open(audio_path, "rb") as f:
                    yield from f

            return StreamingResponse(iterfile(), media_type="audio/mpeg")

        # Otherwise, return metadata JSON
        # Get content and split into chunks using page path directly
        chunks = await split_content_into_chunks(
            content="",  # Not needed when using page_path
            title=page.title,
            description=page.description,
            page_path=page.path,
        )

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

        return JSONResponse(
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

        raise Exception(f"Failed to get audio metadata: {str(e)}") from e


@app.get("/api/audio/{path}/{slug}/{chunk_id}")
async def get_nested_chunk_audio(path: str, slug: str, chunk_id: str, format: str | None = None) -> StreamingResponse:
    """Return audio for a specific chunk of content in a nested folder."""
    # Special case: if chunk_id is the same as slug and format=mp3, treat as a full page request
    if chunk_id == slug and format == "mp3":
        return await get_nested_page_audio(path, slug, format=format)

    if not is_valid_path(path) or not is_valid_slug(slug):
        raise Exception("Invalid path or slug")

    try:
        # Get the content page
        f = safe_path(f"{path}/{slug}/{slug}.md")
        page_obj: Page = _page(f, slug)

        # Get content and split into chunks using page path directly
        chunks = await split_content_into_chunks(
            content="",  # Not needed when using page_path
            title=page_obj.title,
            description=page_obj.description,
            page_path=page_obj.path,
        )

        # Find the requested chunk
        chunk = next((c for c in chunks if c["id"] == chunk_id), None)
        if not chunk:
            raise Exception(f"Chunk {chunk_id} not found")

        # Generate audio path
        audio_dir = get_audio_dir(page_obj.path)
        audio_path = get_audio_path(audio_dir, chunk["checksum"])

        # Use default voice (Will)
        audio_data = get_or_generate_audio(chunk["text"], audio_path)

        return StreamingResponse(audio_data, media_type="audio/mpeg")
    except Exception as e:
        raise Exception(f"Failed to get audio: {str(e)}") from e


@app.get("/api/audio/{path}/{slug}", response_model=None)
async def get_nested_page_audio(
    path: str, slug: str, generate_all: bool | None = None, format: str | None = None
) -> JSONResponse | StreamingResponse:
    """Return audio metadata for a nested page or generate all audio.

    Args:
        path: The folder path
        slug: The slug of the page
        generate_all: Whether to generate all audio chunks
        format: If 'mp3', return a full audio file instead of metadata

    """
    if not is_valid_path(path) or not is_valid_slug(slug):
        raise Exception("Invalid path or slug")

    generate_all = generate_all or False
    format = format or "json"

    try:
        file_path = f"{path}/{slug}/{slug}.md"
        full_path = safe_path(file_path)
        file_exists = cached_file_exists(full_path)

        if not file_exists:
            raise Exception(f"Content file not found: {file_path}")

        # Get the content page
        page_obj: Page = _page(full_path, slug)

        # If MP3 format requested, return full audio file
        if format.lower() == "mp3":
            # Generate or get full audio file
            audio_path, audio_size = await generate_or_get_full_audio(page_obj.path, page_obj.slug)

            # Return streaming response with audio file
            def iterfile() -> Iterator[bytes]:
                with open(audio_path, "rb") as f:
                    yield from f

            return StreamingResponse(iterfile(), media_type="audio/mpeg")

        # Get content and split into chunks using page path directly
        chunks = await split_content_into_chunks(
            content="",  # Not needed when using page_path
            title=page_obj.title,
            description=page_obj.description,
            page_path=page_obj.path,
        )

        audio_dir = get_audio_dir(page_obj.path)

        if generate_all:
            # Generate audio for all chunks
            for chunk in chunks:
                audio_path = get_audio_path(audio_dir, chunk["checksum"])
                get_or_generate_audio(chunk["text"], audio_path)
                chunk["has_audio"] = True
        else:
            # Just check which chunks have audio
            for chunk in chunks:
                audio_path = get_audio_path(audio_dir, chunk["checksum"])
                chunk["has_audio"] = os.path.exists(audio_path)

        return JSONResponse(
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

        raise Exception(f"Failed to get audio metadata: {str(e)}") from e


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
