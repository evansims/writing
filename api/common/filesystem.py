from functools import lru_cache
import os

@lru_cache(maxsize=1024)
def cached_file_exists(path: str) -> bool:
    return os.path.exists(path)

@lru_cache(maxsize=1024)
def cached_file_read(path: str) -> str:
    with open(path, 'r') as f:
        return f.read()
