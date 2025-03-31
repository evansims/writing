import glob
import os

from fastapi import FastAPI
from fastapi.responses import StreamingResponse

from _config import get_llms_config
from _content import _page, _pages
from _filesystem import get_content_dir
from _validation import get_content_path

app = FastAPI()


@app.get("/api/llms")
async def get_llms() -> StreamingResponse:
    """Serve a condensed version of content for LLMs."""
    config = get_llms_config()

    # Get max entries from config, default to 100
    max_entries = config.get("max_entries", 100)
    max_summary_length = config.get("max_summary_length", 500)

    # Get all content files
    content_files = glob.glob(f"{get_content_path()}/**/*.md", recursive=True)
    sorted_files = sorted(content_files, key=os.path.getmtime, reverse=True)[:max_entries]

    entries = []

    for content_file in sorted_files:
        try:
            # Extract slug from file path
            rel_path = os.path.relpath(content_file, get_content_path())
            path = os.path.splitext(rel_path)[0]
            url_path = path.replace("\\", "/")
            file_name = os.path.basename(path)

            # Use _page to process the file
            page = await _page(content_file, file_name)

            # Get created date as string for display
            created_str = page.created.strftime("%Y-%m-%d") if page.created else ""

            # Add summary of the content with truncation
            content_summary = page.body[:max_summary_length]
            if len(page.body) > max_summary_length:
                content_summary += "..."

            entries.append(
                f"URL: /{url_path}\nTitle: {page.title}\nDate: {created_str}\n\n"
                f"{page.description or ''}\n\n{content_summary}\n\n---\n"
            )
        except Exception as e:
            print(f"Error processing {content_file}: {e}")

    return StreamingResponse("".join(entries), media_type="text/plain; charset=utf-8")


@app.get("/api/llms/full")
async def get_llms_full() -> StreamingResponse:
    """Serve a full version of content for LLMs."""
    config = get_llms_config()

    # Get max entries from config, default to 50 for full content (to avoid huge responses)
    max_entries = config.get("max_full_entries", 50)

    # Get all pages from the content directory
    all_pages = await _pages(get_content_dir())

    # Limit the number of pages based on config
    limited_pages = all_pages[:max_entries]

    entries = []

    for page in limited_pages:
        try:
            # Get the URL path
            rel_path = os.path.relpath(page.path, get_content_path())
            path = os.path.splitext(rel_path)[0]
            url_path = path.replace("\\", "/")

            # Get created date as string for display
            created_str = page.created.strftime("%Y-%m-%d") if page.created else ""

            entries.append(
                f"URL: /{url_path}\nTitle: {page.title}\nDate: {created_str}\n\n"
                f"{page.description or ''}\n\n{page.body}\n\n---\n"
            )
        except Exception as e:
            print(f"Error processing {page.path}: {e}")

    return StreamingResponse("".join(entries), media_type="text/plain; charset=utf-8")


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
