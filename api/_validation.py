import os
from functools import lru_cache

from sanic.exceptions import BadRequest

from ._filesystem import get_content_dir


@lru_cache(maxsize=1024)
def is_valid_slug(slug: str) -> bool:
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789-")
    return all(c in allowed for c in slug.lower()) and 3 <= len(slug) <= 64


@lru_cache(maxsize=1024)
def is_valid_path(slug: str) -> bool:
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789-/")
    return all(c in allowed for c in slug.lower()) and 3 <= len(slug) <= 64


@lru_cache(maxsize=1024)
def safe_path(path: str, base_path: str | None = None) -> str:
    if base_path is None:
        base_path = get_content_dir()

    target_path = get_content_dir(path)

    # Debug path resolution issues
    api_debug_logging = os.getenv("API_DEBUG_LOGGING", "false").lower() == "true"
    if api_debug_logging:
        print("SAFE_PATH DEBUG:")
        print(f"  Input path: {path}")
        print(f"  Base path: {base_path}")
        print(f"  Target path: {target_path}")
        print(f"  Base path exists: {os.path.exists(base_path)}")
        print(f"  Target path exists: {os.path.exists(target_path)}")

        # Try direct file access without safe_path for comparison
        direct_path = os.path.join(base_path, path)
        print(f"  Direct path: {direct_path}")
        print(f"  Direct path exists: {os.path.exists(direct_path)}")

    if not os.path.exists(base_path):
        raise BadRequest("Base directory does not exist")

    normalized_base = os.path.normpath(base_path)
    normalized_target = os.path.normpath(target_path)

    if not normalized_target.startswith(normalized_base):
        raise BadRequest("Invalid path traversal attempt")

    return normalized_target
