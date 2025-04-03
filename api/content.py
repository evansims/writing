from fastapi import FastAPI
from fastapi.responses import JSONResponse
from functools import lru_cache

from api._content import _pages
from api._filesystem import get_content_dir

app = FastAPI()


@lru_cache(maxsize=1024)
def _get_content(
    path: str | None = None,
    type: str | None = None,
):
    ps = _pages(get_content_dir())

    if type:
        types = type.split(",")
        ps = [p for p in ps if p.type in types]

    return {"pages": [p.json() for p in ps]}


@app.get("/api/content")
def get_content(
    path: str | None = None,
    type: str | None = None,
) -> JSONResponse:
    return JSONResponse(_get_content(path, type))


# @app.get("/api/content/")
# async def list_content(type: str | None = None) -> StreamingResponse:
#     """List all pages."""
#     ps = await _pages(get_content_dir())

#     if type:
#         types = type.split(",")
#         ps = [p for p in ps if p.type in types]

#     return StreamingResponse({"pages": [p.json() for p in ps]})


# @app.get("/api/content/{path}/")
# async def list_nested_pages(path: str, type: str | None = None) -> StreamingResponse:
#     """List pages in a nested directory."""
#     if not is_valid_path(path):
#         raise Exception("Invalid path")

#     try:
#         f = safe_path(path)
#     except Exception:
#         return StreamingResponse({"pages": []})

#     ps = await _pages(f)

#     if type:
#         types = type.split(",")
#         ps = [p for p in ps if p.type in types]

#     return StreamingResponse({"pages": [p.json() for p in ps]})


# @app.get("/api/content/{path}/{slug}")
# async def get_nested_content(path: str, slug: str) -> StreamingResponse:
#     """Get a page in a nested directory."""
#     if not is_valid_path(path) or not is_valid_slug(slug):
#         raise Exception("Invalid path or slug")

#     f = safe_path(f"{path}/{slug}/{slug}.md")
#     p = await _page(f, slug)

#     return StreamingResponse({"page": p.json()})


# @app.get("/api/content/{path}/{slug}")
# async def get_content(path: str, slug: str) -> StreamingResponse:
#     """Get a page."""
#     if not is_valid_path(path) or not is_valid_slug(slug):
#         raise Exception("Invalid path or slug")

#     f = safe_path(f"{path}/{slug}/{slug}.md")
#     p = await _page(f, slug)

#     return StreamingResponse({"page": p.json()})


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
