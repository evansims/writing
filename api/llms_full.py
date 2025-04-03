from functools import lru_cache
from fastapi import FastAPI
from fastapi.responses import PlainTextResponse

from api._config import get_site_config
from api._content import _pages, ensure_heading_levels
from api._filesystem import get_content_dir

app = FastAPI()


@lru_cache(maxsize=1024)
def _get_llms_full() -> str:
    all_pages = _pages(get_content_dir())

    site_config = get_site_config()

    entries = []

    entries.append(f"# {site_config.get('title')}\n\n> {site_config.get('description')}\n\n")

    for page in all_pages:
        try:
            _title = ""
            _description = ""
            _content = ""

            if page.title is not None:
                _title = f"## {page.title}\n"

            if page.description is not None:
                _description = f"> {page.description}\n\n"

            if page.body is not None:
                _content = ensure_heading_levels(page.body)

            _heading = "\n".join([item for item in [_title, _description] if item is not None])
            _body = f"{_content}\n\n"

            entries.append(f"{_heading}{_body}")
        except Exception as e:
            print(f"Error processing {page.path}: {e}")

    return "---\n\n".join(entries)


@app.get("/api/llms_full")
def get_llms_full() -> PlainTextResponse:
    """Serve a full version of content for LLMs."""

    return PlainTextResponse(_get_llms_full())


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
