from fastapi import FastAPI

from ._content import _page, _pages
from ._filesystem import get_content_dir
from ._validation import is_valid_path, is_valid_slug, safe_path

app = FastAPI()


@app.get("/api/content/")
async def list_content(type: str | None = None) -> dict:
    """List all pages."""
    ps = await _pages(get_content_dir())

    if type:
        types = type.split(",")
        ps = [p for p in ps if p.type in types]

    return {
        "pages": [p.json() for p in ps],
    }


@app.get("/api/content/{path}/")
async def list_nested_pages(path: str, type: str | None = None) -> dict:
    """List pages in a nested directory."""
    if not is_valid_path(path):
        raise Exception("Invalid path")

    try:
        f = safe_path(path)
    except Exception:
        return {"pages": []}

    ps = await _pages(f)

    if type:
        types = type.split(",")
        ps = [p for p in ps if p.type in types]

    return {
        "pages": [p.json() for p in ps],
    }


@app.get("/api/content/{path}/{slug}")
async def get_nested_content(path: str, slug: str) -> dict:
    """Get a page in a nested directory."""
    if not is_valid_path(path) or not is_valid_slug(slug):
        raise Exception("Invalid path or slug")

    f = safe_path(f"{path}/{slug}/{slug}.md")
    p = await _page(f, slug)

    return {"page": p.json()}


@app.get("/api/content/{path}/{slug}")
async def get_content(path: str, slug: str) -> dict:
    """Get a page."""
    if not is_valid_path(path) or not is_valid_slug(slug):
        raise Exception("Invalid path or slug")

    f = safe_path(f"{path}/{slug}/{slug}.md")
    p = await _page(f, slug)

    return {"page": p.json()}
