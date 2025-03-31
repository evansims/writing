import os
from functools import lru_cache


@lru_cache(maxsize=1024)
def cached_file_exists(path: str) -> bool:
    return os.path.exists(path)


@lru_cache(maxsize=1024)
def cached_file_read(path: str) -> str:
    with open(path) as f:
        return f.read()


@lru_cache(maxsize=1024)
def get_content_dir(path: str | None = None) -> str:
    parent_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    if path is None:
        # Return the content directory in the parent directory
        return os.path.abspath(os.path.join(parent_dir, "content"))
    else:
        # Return the content path in the parent directory
        return os.path.abspath(os.path.join(parent_dir, "content", path))
