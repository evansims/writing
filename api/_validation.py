import os
from functools import lru_cache

from fastapi import HTTPException

from api._filesystem import get_content_dir


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

    if not os.path.exists(base_path):
        raise HTTPException(status_code=400, detail="Base directory does not exist")

    normalized_base = os.path.normpath(base_path)
    normalized_target = os.path.normpath(target_path)

    if not normalized_target.startswith(normalized_base):
        raise HTTPException(status_code=400, detail="Invalid path traversal attempt")

    return normalized_target
