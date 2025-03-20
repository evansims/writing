import os
from functools import lru_cache

from sanic.exceptions import BadRequest


@lru_cache(maxsize=1024)
def get_content_path(path: str | None = None) -> str:
    if path is None:
        return os.path.abspath(os.path.join(os.getcwd(), "content"))
    else:
        return os.path.abspath(os.path.join(os.getcwd(), "content", path))

@lru_cache(maxsize=1024)
def validate_slug(slug: str) -> bool:
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789-")
    return all(c in allowed for c in slug.lower()) and 3 <= len(slug) <= 64

@lru_cache(maxsize=1024)
def safe_path(slug: str) -> str:
    if not validate_slug(slug):
        raise BadRequest("Invalid slug format")

    base_path = get_content_path()
    target_path = get_content_path(f"{slug}.md")

    if os.path.commonpath([base_path, target_path]) != base_path:
        raise BadRequest("Invalid path traversal attempt")

    return target_path
