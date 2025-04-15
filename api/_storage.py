"""Helper module for interacting with Vercel's blog storage APIs.

This module provides a clean interface for reading, writing, and deleting file data
from Vercel's blog storage system.
"""

import asyncio
import os
from typing import Union
from urllib.parse import quote, urljoin

import dotenv
import httpx

# Constants for Vercel Blob API
API_VERSION = "4"
DEFAULT_RETRY_ATTEMPTS = 3
DEFAULT_RETRY_DELAY = 1  # seconds
DEFAULT_CACHE_AGE = "31536000"  # 1 year in seconds


class StorageError(Exception):
    """Base exception for storage-related errors."""

    pass


class StorageClient:
    """Client for interacting with Vercel's blog storage APIs."""

    def __init__(self, base_url: str | None = None, token: str | None = None) -> None:
        """Initialize the storage client.

        Args:
            base_url: Optional base URL for the storage API. Defaults to env var.
            token: Optional auth token. Defaults to env var.

        """
        dotenv.load_dotenv()

        self.base_url = base_url or os.getenv("BLOB_BASE_URL")
        if not self.base_url:
            raise StorageError("No storage URL provided or found in environment")

        self.token = token or os.getenv("BLOB_READ_WRITE_TOKEN")
        if not self.token:
            raise StorageError("No auth token provided or found in environment")

        # Base headers that are always needed
        self.base_headers = {
            "Authorization": f"Bearer {self.token}",
            "x-api-version": API_VERSION,
        }

    def _guess_mime_type(self, path: str) -> str:
        """Guess the MIME type based on file extension."""
        mime_types = {".mp3": "audio/mpeg", ".json": "application/json", ".txt": "text/plain"}
        ext = os.path.splitext(path)[1].lower()
        return mime_types.get(ext, "application/octet-stream")

    def _clean_path(self, path: str) -> str:
        """Clean and encode the path for storage."""
        # Remove leading/trailing slashes
        path = path.strip("/")
        # Normalize path separators to forward slashes
        path = path.replace("\\", "/")
        # URL encode each path segment while preserving slashes
        segments = path.split("/")
        encoded_segments = [quote(segment, safe="") for segment in segments]
        return "/".join(encoded_segments)

    async def read_file(self, path: str) -> bytes:
        """Read a file from storage.

        Args:
            path: Path to the file relative to the storage root

        Returns:
            bytes: The file contents as bytes

        Raises:
            StorageError: If the file cannot be read
        """
        path = self._clean_path(path)
        url = urljoin(self.base_url, path)

        headers = {
            "Authorization": f"Bearer {self.token}",
            "x-api-version": API_VERSION,
        }

        for attempt in range(DEFAULT_RETRY_ATTEMPTS):
            try:
                async with httpx.AsyncClient() as client:
                    response = await client.get(url, headers=headers)

                    # Try to get detailed error information
                    error_details = None
                    try:
                        error_data = response.json()
                        error_details = error_data.get("error", {}).get("message")
                    except Exception:
                        error_details = response.text

                    if response.status_code == 200:
                        return response.content
                    elif response.status_code == 503:
                        if attempt < DEFAULT_RETRY_ATTEMPTS - 1:
                            await asyncio.sleep(DEFAULT_RETRY_DELAY * (2**attempt))
                            continue

                    # Handle specific error cases
                    if response.status_code == 400:
                        raise StorageError(f"Bad request: {error_details}")
                    elif response.status_code == 401:
                        raise StorageError("Unauthorized: Invalid token")
                    elif response.status_code == 404:
                        raise StorageError(f"File not found: {path}")
                    else:
                        raise StorageError(f"Storage error (HTTP {response.status_code}): {error_details}")

            except httpx.RequestError as e:
                if attempt < DEFAULT_RETRY_ATTEMPTS - 1:
                    await asyncio.sleep(DEFAULT_RETRY_DELAY * (2**attempt))
                    continue
                raise StorageError(f"Network error reading file: {e}") from e

        raise StorageError("Max retry attempts reached")

    async def write_file(self, path: str, data: Union[str, bytes]) -> None:
        """Write data to a file in storage.

        Args:
            path: The path to write to
            data: The data to write (string or bytes)

        Raises:
            StorageError: If there is an error writing to storage
        """
        # Clean and prepare the path
        path = self._clean_path(path)

        # Convert string data to bytes if needed
        if isinstance(data, str):
            data = data.encode("utf-8")

        # Set up headers based on the Vercel Blob API requirements
        headers = {
            "Authorization": f"Bearer {self.token}",
            "access": "public",  # Required for public access
            "x-api-version": API_VERSION,
            "x-content-type": self._guess_mime_type(path),
            "x-cache-control-max-age": DEFAULT_CACHE_AGE,
        }

        # Attempt the upload with retries
        for attempt in range(DEFAULT_RETRY_ATTEMPTS):
            try:
                async with httpx.AsyncClient() as client:
                    # PUT directly to the path, NOT to /store
                    url = f"{self.base_url}/{path}"
                    response = await client.put(url, headers=headers, content=data)

                    # Try to get detailed error information
                    error_details = None
                    try:
                        error_data = response.json()
                        error_details = error_data.get("error", {}).get("message")
                    except Exception:
                        error_details = response.text

                    if response.status_code == 200:
                        return
                    elif response.status_code == 503:
                        if attempt < DEFAULT_RETRY_ATTEMPTS - 1:
                            await asyncio.sleep(DEFAULT_RETRY_DELAY * (2**attempt))
                            continue

                    # Handle specific error cases
                    if response.status_code == 400:
                        raise StorageError(f"Bad request: {error_details}")
                    elif response.status_code == 401:
                        raise StorageError("Unauthorized: Invalid token")
                    elif response.status_code == 413:
                        raise StorageError("File too large")
                    else:
                        raise StorageError(f"Storage error (HTTP {response.status_code}): {error_details}")

            except httpx.RequestError as e:
                if attempt < DEFAULT_RETRY_ATTEMPTS - 1:
                    await asyncio.sleep(DEFAULT_RETRY_DELAY * (2**attempt))
                    continue
                raise StorageError(f"Network error writing file: {e}") from e

        raise StorageError("Max retry attempts reached")

    async def delete_file(self, path: str) -> None:
        """Delete a file from storage.

        Args:
            path: Path to the file to delete.

        Raises:
            StorageError: If the file cannot be deleted.
        """
        clean_path = self._clean_path(path)
        file_url = urljoin(self.base_url + "/", clean_path)

        async with httpx.AsyncClient() as client:
            try:
                # Use POST with urls array as specified in Vercel Blob API
                response = await client.post(
                    f"{self.base_url}/delete",
                    headers=self.base_headers,
                    json={"urls": [file_url]},
                )

                if response.status_code != 200:
                    raise StorageError(f"Failed to delete file: {path} (Status: {response.status_code})")
            except httpx.RequestError as e:
                raise StorageError(f"Network error deleting file: {e}") from e


# Create a default client instance
storage = StorageClient()
