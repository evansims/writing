from fastapi import FastAPI
from fastapi.responses import StreamingResponse

from api._config import get_site_config
from api._content import _pages, ensure_heading_levels
from api._filesystem import get_content_dir

app = FastAPI()


@app.get("/api/llms-full")
async def get_llms_full() -> StreamingResponse:
    """Serve a full version of content for LLMs."""
    all_pages = await _pages(get_content_dir())

    site_config = get_site_config()

    entries = []

    entries.append(f"# {site_config.get('title')}\n> {site_config.get('description')}\n\n")

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

    return StreamingResponse("---\n\n".join(entries), media_type="text/plain; charset=utf-8")


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
