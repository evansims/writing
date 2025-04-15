from functools import lru_cache

from fastapi import FastAPI, HTTPException
from fastapi.responses import JSONResponse

from api._content import _pages
from api._filesystem import get_content_dir
from api._validation import is_valid_path, safe_path

app = FastAPI()


@lru_cache(maxsize=1024)
def _get_content(
    path: str | None = None,
    type: str | None = None,
) -> JSONResponse:
    if path:
        if not is_valid_path(path):
            raise HTTPException(status_code=400, detail="Invalid path")

        try:
            f = safe_path(path)
        except Exception:
            return {"pages": []}

        ps = _pages(f)
    else:
        ps = _pages(get_content_dir())

    if type:
        types = type.split(",")
        ps = [p for p in ps if p.type in types]

    return JSONResponse({"pages": [p.json() for p in ps]})


@app.get("/api/content")
def get_content(
    path: str | None = None,
    type: str | None = None,
) -> JSONResponse:
    """Get content from the content directory."""
    return _get_content(path, type)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
