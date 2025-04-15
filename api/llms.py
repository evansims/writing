from functools import lru_cache

from fastapi import FastAPI
from fastapi.responses import PlainTextResponse

from api._config import get_site_config
from api._content import _pages
from api._filesystem import get_content_dir

app = FastAPI()


@lru_cache(maxsize=1024)
def _get_llms() -> str:
    all_pages = _pages(get_content_dir())

    site_config = get_site_config()
    site_url = site_config.get("url").rstrip("/")

    entries = []

    entries.append(f"# {site_config.get('title')}\n\n> {site_config.get('description')}\n")

    for page in all_pages:
        try:
            _url = page.url()
            _description = f": {page.description}" if page.description else ""

            entries.append(f"- [{page.title}]({site_url}{_url}){_description}")
        except Exception as e:
            print(f"Error processing {page.path}: {e}")

    return "\n".join(entries)


@app.get("/api/llms")
def get_llms() -> PlainTextResponse:
    """Serve a condensed sitemap for LLMs."""
    return PlainTextResponse(_get_llms())


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
