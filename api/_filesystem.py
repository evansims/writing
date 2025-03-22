from functools import lru_cache
import os

@lru_cache(maxsize=1024)
def cached_file_exists(path: str) -> bool:
    return os.path.exists(path)

@lru_cache(maxsize=1024)
def cached_file_read(path: str) -> str:
    with open(path, 'r') as f:
        return f.read()

@lru_cache(maxsize=1024)
def get_content_dir(path: str | None = None) -> str:
    if path is None:
        return os.path.abspath(os.path.join(os.getcwd(), "content"))
    else:
        return os.path.abspath(os.path.join(os.getcwd(), "content", path))