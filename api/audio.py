import asyncio
import contextlib
import hashlib
import logging
import os
import re
import time
from collections.abc import AsyncGenerator, AsyncIterator, Awaitable, Callable
from typing import Any

import dotenv
from elevenlabs.client import ElevenLabs
from fastapi import BackgroundTasks, FastAPI, HTTPException, Request, Response, status
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse, StreamingResponse
from pydantic import BaseModel

from api._content import _page
from api._filesystem import cached_file_exists
from api._storage import StorageError, storage
from api._types import Page
from api._validation import is_valid_slug, safe_path

# Configure logging
logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(name)s - %(levelname)s - %(message)s")
logger = logging.getLogger(__name__)

app = FastAPI()

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Adjust this in production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

dotenv.load_dotenv()

# Load environment variables
API_KEY = os.getenv("ELEVENLABS_API_KEY")
VOICE_ID = os.getenv("ELEVENLABS_VOICE_ID") or "bIHbv24MWmeRgasZH58o"
MODEL_ID = os.getenv("ELEVENLABS_MODEL_ID") or "eleven_multilingual_v2"

# Initialize Eleven Labs client
client = ElevenLabs(api_key=API_KEY)

# File lock for preventing concurrent file operations on the same file
file_locks: dict[str, asyncio.Lock] = {}

# Rate limiting settings
RATE_LIMIT_CALLS = int(os.getenv("RATE_LIMIT_CALLS", "60"))  # Calls per minute
RATE_LIMIT_WINDOW = 60  # 1 minute window in seconds
rate_limit_data: dict[str, float | int] = {"calls": 0, "window_start": time.time()}


# Pydantic models for request/response validation
class AudioMetadataResponse(BaseModel):
    """Response model for audio metadata."""

    page: dict[str, Any]
    chunks: list[dict[str, Any]]


class ChunkMetadata(BaseModel):
    """Model for chunk metadata."""

    id: str
    text: str
    checksum: str
    has_audio: bool
    title: str | None = None


# Concurrency control with a semaphore to limit simultaneous API calls
API_SEMAPHORE = asyncio.Semaphore(3)  # Limit to 3 concurrent API calls


# Rate limiter middleware
@app.middleware("http")
async def rate_limiter(request: Request, call_next: Callable[[Request], Awaitable[Response]]) -> Response:
    """Middleware to implement rate limiting."""
    global rate_limit_data

    # Reset counter if window has elapsed
    current_time = time.time()
    if current_time - rate_limit_data["window_start"] > RATE_LIMIT_WINDOW:
        rate_limit_data = {"calls": 0, "window_start": current_time}

    # Check if limit exceeded
    if rate_limit_data["calls"] >= RATE_LIMIT_CALLS:
        return JSONResponse(
            status_code=status.HTTP_429_TOO_MANY_REQUESTS,
            content={"detail": "Rate limit exceeded. Please try again later."},
        )

    # Increment counter and process request
    rate_limit_data["calls"] += 1
    return await call_next(request)


# Error handling middleware
@app.middleware("http")
async def error_handling_middleware(request: Request, call_next: Callable[[Request], Awaitable[Response]]) -> Response:
    """Middleware to provide consistent error handling."""
    try:
        return await call_next(request)
    except Exception as e:
        logger.error(f"Unhandled exception: {str(e)}")
        import traceback

        logger.error(traceback.format_exc())

        if isinstance(e, HTTPException):
            # Pass through HTTP exceptions
            raise e

        # Convert generic exceptions to HTTPException
        return JSONResponse(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            content={"detail": f"An unexpected error occurred: {str(e)}"},
        )


@contextlib.asynccontextmanager
async def file_lock(file_path: str) -> AsyncIterator[None]:
    """Async context manager for file locks to prevent race conditions."""
    if file_path not in file_locks:
        file_locks[file_path] = asyncio.Lock()

    lock = file_locks[file_path]
    try:
        await lock.acquire()
        yield
    finally:
        if lock.locked():
            lock.release()
        # Clean up if no one is waiting
        if not lock.locked():
            file_locks.pop(file_path, None)


def get_storage_path(file_path: str) -> str:
    """Convert filesystem path to storage path.

    This function ensures that audio files maintain their directory structure
    relative to the content directory, preserving the audio subdirectory.

    Args:
        file_path: The path to convert to a storage path

    Returns:
        str: The storage path for the audio file

    """
    # Get the path relative to the content directory
    content_dir = os.path.join(os.path.dirname(__file__), "..", "content")

    try:
        # Ensure the path is absolute before getting relative path
        abs_path = os.path.abspath(file_path)
        rel_path = os.path.relpath(abs_path, content_dir)

        # Split the path into components
        parts = rel_path.split(os.sep)

        # For audio files, ensure they go in an audio subdirectory
        if not any(p == "audio" for p in parts):
            # If no audio directory in path, add it
            parts.insert(-1, "audio")

        # Normalize path separators for storage
        return "/".join(parts).replace("\\", "/")

    except ValueError:
        # If the path is not under content_dir, ensure it still goes in audio dir
        parts = file_path.split(os.sep)
        if not any(p == "audio" for p in parts):
            # Add audio directory before the filename
            parts.insert(-1, "audio")
        return "/".join(parts).replace("\\", "/")


def get_audio_dir(content_path: str) -> str:
    """Get the audio directory path for a given content path.

    This is now used primarily for path construction, not filesystem operations.
    """
    content_dir = os.path.dirname(content_path)
    return os.path.join(content_dir, "audio")


async def split_content_into_chunks(
    content: str = "", title: str | None = None, description: str | None = None, page_path: str | None = None
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
            logger.error(f"Error loading page from path {page_path}: {e}")
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR, detail=f"Error loading page content: {str(e)}"
            ) from e

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
    # First, capture header text and add pauses after them
    clean_text = re.sub(r"^(#+)\s+(.+)$", r"\2. . . .", text, flags=re.MULTILINE)

    # Remove any remaining markdown formatting
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
    async with file_lock(audio_path):
        try:
            # Try to read from storage first
            storage_path = get_storage_path(audio_path)
            audio_bytes = await storage.read_file(storage_path)
            if audio_bytes:
                yield audio_bytes
                return
        except StorageError:
            # File doesn't exist or is empty, continue to generation
            pass

        # Generate new audio with rate limiting
        try:
            logger.info(f"Generating audio for text: '{chunk_text[:50]}...' using Eleven Labs API")
            logger.info(f"Using voice_id={VOICE_ID}, model_id={MODEL_ID}")

            # Check if API key is available
            if not API_KEY:
                raise HTTPException(
                    status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
                    detail="Text-to-speech service not configured. Missing API key.",
                )

            # Add exponential backoff retry logic for API calls
            max_retries = 3
            retry_delay = 1  # starting delay in seconds

            for attempt in range(max_retries):
                try:
                    async with API_SEMAPHORE:
                        # Use the text_to_speech.convert method from the client
                        audio_data = client.text_to_speech.convert(
                            text=chunk_text,
                            voice_id=VOICE_ID,
                            model_id=MODEL_ID,
                            output_format="mp3_44100_128",
                        )

                        # Handle if audio_data is a generator
                        if hasattr(audio_data, "__iter__") and not isinstance(audio_data, bytes | bytearray):
                            logger.info("Converting generator to bytes")
                            audio_bytes = b"".join(chunk for chunk in audio_data)
                        else:
                            audio_bytes = audio_data

                        # Check if we got data back
                        if not audio_bytes or len(audio_bytes) == 0:
                            raise ValueError("Received empty audio data from Eleven Labs API")

                        # Success, break the retry loop
                        break

                except Exception as e:
                    if attempt < max_retries - 1:
                        # Log the error and retry
                        logger.warning(
                            f"API call attempt {attempt + 1} failed: {str(e)}. Retrying in {retry_delay}s..."
                        )
                        await asyncio.sleep(retry_delay)
                        retry_delay *= 2  # Exponential backoff
                    else:
                        # Last attempt failed, re-raise
                        raise
            else:
                # If we get here, all retries failed
                raise HTTPException(
                    status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
                    detail=f"Text-to-speech service unavailable after {max_retries} attempts",
                )

            logger.info(f"Successfully generated {len(audio_bytes)} bytes of audio data")

            # Store the generated audio
            try:
                storage_path = get_storage_path(audio_path)
                await storage.write_file(storage_path, audio_bytes)
                logger.info(f"Saved audio file to {storage_path}")
            except StorageError as file_error:
                logger.error(f"Error saving audio file: {str(file_error)}")
                # Continue even if saving fails - we still have the audio data

            yield audio_bytes
        except HTTPException:
            # Pass through HTTP exceptions
            raise
        except Exception as e:
            logger.error(f"Error generating audio: {str(e)}")
            import traceback

            logger.error(traceback.format_exc())
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR, detail=f"Failed to generate audio: {str(e)}"
            ) from e


async def generate_or_get_full_audio(page_path: str, page_slug: str) -> tuple[str, int]:
    """Generate or get a full audio file from all chunks.

    Returns:
        tuple[str, int]: Path to the full audio file and its size in bytes

    Raises:
        HTTPException: If there is an error generating the full audio

    """
    try:
        # Get audio directory
        audio_dir = get_audio_dir(page_path)
        full_audio_path = get_full_audio_path(audio_dir, page_slug)

        async with file_lock(full_audio_path):
            try:
                # Check if full audio already exists
                storage_path = get_storage_path(full_audio_path)
                audio_data = await storage.read_file(storage_path)
                return full_audio_path, len(audio_data)
            except StorageError:
                # File doesn't exist or is empty, continue to generation
                pass

            # Get all chunks and generate audio for each one if needed
            chunks = await split_content_into_chunks(content="", page_path=page_path)

            # Check if all chunks have audio files and generate missing ones concurrently
            audio_paths = []
            generation_tasks = []

            for chunk in chunks:
                chunk_audio_path = get_audio_path(audio_dir, chunk.get("checksum", ""))
                try:
                    storage_path = get_storage_path(chunk_audio_path)
                    await storage.read_file(storage_path)
                except StorageError:
                    # Schedule generation of missing audio
                    generation_tasks.append(generate_chunk_audio(chunk["text"], chunk_audio_path))

                audio_paths.append(chunk_audio_path)

            # Wait for all audio generation tasks to complete if any
            if generation_tasks:
                await asyncio.gather(*generation_tasks)

            # Ensure all audio files exist
            for audio_path in audio_paths:
                try:
                    storage_path = get_storage_path(audio_path)
                    await storage.read_file(storage_path)
                except StorageError as e:
                    logger.error(f"Error reading audio file: {str(e)}")
                    raise HTTPException(
                        status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                        detail="Failed to generate all required audio chunks",
                    ) from e

            # Concatenate audio files
            all_audio = b""
            for audio_path in audio_paths:
                storage_path = get_storage_path(audio_path)
                chunk_audio = await storage.read_file(storage_path)
                all_audio += chunk_audio

            # Save the concatenated file
            storage_path = get_storage_path(full_audio_path)
            await storage.write_file(storage_path, all_audio)

            return full_audio_path, len(all_audio)

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error generating full audio: {str(e)}")
        import traceback

        logger.error(traceback.format_exc())
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR, detail=f"Failed to generate full audio: {str(e)}"
        ) from e


async def generate_chunk_audio(text: str, audio_path: str) -> None:
    """Generate audio for a chunk of text.

    Args:
        text: The text to generate audio for
        audio_path: The path to save the audio file to

    Raises:
        HTTPException: If there is an error generating the audio

    """
    try:
        async with file_lock(audio_path):
            try:
                # Check if audio already exists in storage
                storage_path = get_storage_path(audio_path)
                await storage.read_file(storage_path)
                return
            except StorageError:
                # File doesn't exist or is empty, continue to generation
                pass

            # Generate audio using ElevenLabs client
            audio_data = client.text_to_speech.convert(
                text=text,
                voice_id=VOICE_ID,
                model_id=MODEL_ID,
                output_format="mp3_44100_128",
            )

            # Handle if audio_data is a generator
            if hasattr(audio_data, "__iter__") and not isinstance(audio_data, bytes | bytearray):
                logger.info("Converting generator to bytes")
                audio_bytes = b"".join(chunk for chunk in audio_data)
            else:
                audio_bytes = audio_data

            # Check if we got data back
            if not audio_bytes or len(audio_bytes) == 0:
                raise ValueError("Received empty audio data from Eleven Labs API")

            # Save to storage
            storage_path = get_storage_path(audio_path)
            await storage.write_file(storage_path, audio_bytes)

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error generating chunk audio: {str(e)}")
        import traceback

        logger.error(traceback.format_exc())
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to generate chunk audio: {str(e)}",
        ) from e


async def concatenate_mp3_files(audio_paths: list[str], output_path: str) -> None:
    """Concatenate MP3 files properly, handling headers and footers.

    For MP3 files from the same source (like ElevenLabs), this approach works well.
    For a more robust solution in the future, consider using pydub or another audio library.
    """
    try:
        with open(output_path, "wb") as outfile:
            for audio_path in audio_paths:
                with open(audio_path, "rb") as infile:
                    outfile.write(infile.read())
    except Exception as e:
        logger.error(f"Error concatenating MP3 files: {str(e)}")
        # Clean up the partial file
        if os.path.exists(output_path):
            with contextlib.suppress(Exception):
                os.remove(output_path)
        raise


async def get_content_page(path_or_slug: str, nested_slug: str | None = None) -> Page:
    """Get content page from path or slug."""
    try:
        # If nested_slug contains slashes, it's likely a mistake in path handling
        if nested_slug and "/" in nested_slug:
            logger.warning(f"Potential path handling issue - nested_slug contains slashes: {nested_slug}")
            # Handle this case by splitting nested_slug and using only the last part
            nested_slug = nested_slug.split("/")[-1]

        # Handle multi-level paths
        if "/" in path_or_slug and not nested_slug:
            parts = path_or_slug.split("/")
            # For paths like "strategy/tiny-changes/tiny-changes"
            if len(parts) >= 3 and parts[-1] == parts[-2]:
                # Handle the special case where the last segment is repeated
                top_level = parts[0]
                middle = parts[-1]
                file_path = f"{top_level}/{middle}/{middle}.md"
                logger.info(f"Looking for file at path: {file_path}")
                full_path = safe_path(file_path)

                if cached_file_exists(full_path):
                    return _page(full_path, middle)

            # For paths like "strategy/tiny-changes"
            if len(parts) == 2:
                # Simple nested case
                top_level = parts[0]
                page_name = parts[1]
                file_path = f"{top_level}/{page_name}/{page_name}.md"
                logger.info(f"Looking for file at path: {file_path}")
                full_path = safe_path(file_path)

                if cached_file_exists(full_path):
                    return _page(full_path, page_name)

            # If we couldn't resolve a special case, throw 404
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND, detail=f"Content file not found for path: {path_or_slug}"
            )

        if nested_slug:
            # Handle nested page
            file_path = f"{path_or_slug}/{nested_slug}/{nested_slug}.md"
            logger.info(f"Looking for nested file at path: {file_path}")
            full_path = safe_path(file_path)

            if not cached_file_exists(full_path):
                raise HTTPException(
                    status_code=status.HTTP_404_NOT_FOUND, detail=f"Content file not found: {file_path}"
                )

            return _page(full_path, nested_slug)
        else:
            # Handle simple slug
            if not is_valid_slug(path_or_slug):
                raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail="Invalid slug")

            file_path = f"{path_or_slug}/{path_or_slug}.md"
            logger.info(f"Looking for simple file at path: {file_path}")
            full_path = safe_path(file_path)

            if not cached_file_exists(full_path):
                raise HTTPException(
                    status_code=status.HTTP_404_NOT_FOUND, detail=f"Content file not found: {file_path}"
                )

            return _page(full_path, path_or_slug)
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error getting content page: {str(e)}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR, detail=f"Error accessing content: {str(e)}"
        ) from e


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
        logger.error(f"Failed to check audio API health: {str(e)}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR, detail=f"Failed to check audio API health: {str(e)}"
        ) from e


@app.get("/api/audio/{slug}/metadata", response_model=AudioMetadataResponse)
async def get_audio_metadata(slug: str) -> JSONResponse:
    """Return metadata about available audio chunks for a page."""
    page = await get_content_page(slug)

    try:
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
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Failed to get audio metadata: {str(e)}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR, detail=f"Failed to get audio metadata: {str(e)}"
        ) from e


@app.get("/api/audio/{slug}/{chunk_id}")
async def get_chunk_audio(slug: str, chunk_id: str) -> StreamingResponse:
    """Return audio for a specific chunk of content."""
    try:
        # Try to see if this is actually a folder/page request without a chunk ID
        file_path = f"{slug}/{chunk_id}/{chunk_id}.md"
        full_path = safe_path(file_path)

        if cached_file_exists(full_path):
            # This is actually a page request, not a chunk request
            # Redirect to the nested page endpoint
            return await get_nested_page_audio(slug, chunk_id)

        # Get the page content
        page = await get_content_page(slug)

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
            raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail=f"Chunk {chunk_id} not found")

        # Generate audio path
        audio_dir = get_audio_dir(page.path)
        audio_path = get_audio_path(audio_dir, chunk["checksum"])

        # Get or generate audio
        audio_data = get_or_generate_audio(chunk["text"], audio_path)

        return StreamingResponse(audio_data, media_type="audio/mpeg")

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error in get_chunk_audio: {str(e)}")
        import traceback

        logger.error(traceback.format_exc())

        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR, detail=f"Failed to get audio: {str(e)}"
        ) from e


@app.get("/api/audio/{path:path}", response_model=None)
async def get_page_audio(
    path: str, generate_all: bool = False, format: str = "json", background_tasks: BackgroundTasks = None
) -> JSONResponse | StreamingResponse:
    """Retrieve page audio for both nested and non-nested pages."""
    # Extract the components from the path
    path_parts = path.split("/")

    # Special case for the triple segment pattern "strategy/tiny-changes/tiny-changes"
    if len(path_parts) >= 3 and path_parts[-1] == path_parts[-2]:
        # Direct route to handle the common pattern
        directory = path_parts[0]
        page = path_parts[-1]
        logger.info(f"Special case handling for repeated segments: {directory}/{page}")
        return await get_page_audio_impl(directory, page, generate_all, format, background_tasks)

    # Handle different path structures
    if len(path_parts) == 1:
        # Single segment (slug only)
        return await get_page_audio_impl(path_parts[0], None, generate_all, format, background_tasks)
    elif len(path_parts) == 2:
        # Two segments (directory/slug)
        return await get_page_audio_impl(path_parts[0], path_parts[1], generate_all, format, background_tasks)
    elif len(path_parts) >= 3:
        # For other multi-level paths, use only first and last segments
        # This avoids passing complex nested paths that confuse the file resolution
        directory = path_parts[0]
        page = path_parts[-1]
        logger.info(f"Multi-level path handling: using {directory}/{page}")
        return await get_page_audio_impl(directory, page, generate_all, format, background_tasks)


async def get_page_audio_impl(
    path_or_slug: str,
    nested_slug: str | None = None,
    generate_all: bool = False,
    format: str = "json",
    background_tasks: BackgroundTasks = None,
) -> JSONResponse | StreamingResponse:
    """Retrieve page audio for both nested and non-nested pages.

    Args:
        path_or_slug: The path or slug of the page
        nested_slug: If provided, indicates this is a nested page
        generate_all: Whether to generate all audio chunks
        format: If 'mp3', return a full audio file instead of metadata
        background_tasks: FastAPI background tasks for async processing

    Returns:
        JSONResponse or StreamingResponse

    Raises:
        HTTPException: If there is an error getting the page audio

    """
    try:
        # Get the content page
        page = await get_content_page(path_or_slug, nested_slug)

        # If MP3 format requested, return full audio file
        if format.lower() == "mp3":
            # Generate or get full audio file
            audio_path, audio_size = await generate_or_get_full_audio(page.path, page.slug)

            # Return streaming response with audio file
            audio_data = await storage.read_file(audio_path)
            return StreamingResponse(
                iter([audio_data]),
                media_type="audio/mpeg",
                headers={
                    "Content-Disposition": f'attachment; filename="{page.slug}.mp3"',
                    "Content-Length": str(audio_size),
                },
            )

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
            generation_tasks = []
            for chunk in chunks:
                audio_path = get_audio_path(audio_dir, chunk["checksum"])
                # If we have background tasks, use them
                if background_tasks:
                    background_tasks.add_task(generate_chunk_audio, chunk["text"], audio_path)
                    chunk["has_audio"] = "pending"  # Mark as pending generation
                else:
                    # Schedule generation (will complete before response)
                    generation_tasks.append(generate_chunk_audio(chunk["text"], audio_path))
                    chunk["has_audio"] = True

            # If we're not using background tasks, wait for all to complete
            if generation_tasks and not background_tasks:
                await asyncio.gather(*generation_tasks)
        else:
            # Just check which chunks have audio
            for chunk in chunks:
                audio_path = get_audio_path(audio_dir, chunk["checksum"])
                chunk["has_audio"] = os.path.exists(audio_path) and os.path.getsize(audio_path) > 0

        return JSONResponse(
            {
                "page": {
                    "slug": page.slug,
                    "title": page.title,
                },
                "chunks": chunks,
            }
        )
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error in get_page_audio: {str(e)}")
        import traceback

        logger.error(traceback.format_exc())

        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR, detail=f"Failed to get audio metadata: {str(e)}"
        ) from e


# Alias the nested page and chunk functions to the unified implementations
get_nested_page_audio = get_page_audio_impl
get_nested_chunk_audio = get_chunk_audio


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
