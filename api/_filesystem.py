import glob
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


@lru_cache(maxsize=1024)
def get_config_path(config_name: str) -> str:
    """Get the path to a configuration file.

    Args:
        config_name: The name of the configuration file (e.g., 'site.yml')

    Returns:
        The absolute path to the configuration file

    """
    parent_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    return os.path.abspath(os.path.join(parent_dir, "configuration", config_name))


def get_sorted_content_files(max_files: int = None, file_extension: str = "md") -> list[str]:
    """Get content files sorted by modification time (newest first).

    Args:
        max_files: Maximum number of files to return (default: all files)
        file_extension: File extension to filter by, without the dot (default: "md")

    Returns:
        List of file paths sorted by modification time (newest first)

    """
    content_path = get_content_dir()
    content_files = glob.glob(f"{content_path}/**/*.{file_extension}", recursive=True)
    sorted_files = sorted(content_files, key=os.path.getmtime, reverse=True)

    if max_files is not None:
        sorted_files = sorted_files[:max_files]

    return sorted_files
