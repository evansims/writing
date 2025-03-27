import os
from datetime import datetime

import frontmatter
from sanic import Sanic
from sanic.exceptions import NotFound
from sanic.request import Request
from sanic.response import JSONResponse, json

from ._filesystem import cached_file_exists, cached_file_read, get_content_dir
from ._types import Page, ReadingItem
from ._validation import is_valid_path, is_valid_slug, safe_path

app = Sanic(
    name="content_api",
    strict_slashes=True,
)


@app.get("/api/content/")
async def list_pages(request: Request) -> JSONResponse:
    content_type = request.args.get("type")
    ps = await _pages(get_content_dir())

    if content_type:
        types = content_type.split(",")
        ps = [p for p in ps if p.type in types]

    return json(
        {
            "pages": [p.json() for p in ps],
        }
    )


@app.get("/api/content/<folder:path>/")
async def list_nested_pages(request: Request, folder) -> JSONResponse:
    if not is_valid_path(folder):
        raise NotFound()

    try:
        f = safe_path(folder)
    except Exception:
        return json([])

    content_type = request.args.get("type")
    ps = await _pages(f)

    if content_type:
        types = content_type.split(",")
        ps = [p for p in ps if p.type in types]

    return json(
        {
            "pages": [p.json() for p in ps],
        }
    )


@app.get("/api/content/<folder:path>/<page:slug>")
async def get_nested_content(
    request: Request,
    folder: str,
    page: str,
) -> JSONResponse:
    if not is_valid_path(folder) or not is_valid_slug(page):
        raise NotFound()

    # Use the correct path format
    f = safe_path(f"{folder}/{page}/{page}.md")
    p = await _page(f, page)

    return json({"page": p.json()})


@app.get("/api/content/<page:slug>")
async def get_content(
    request: Request,
    page: str,
) -> JSONResponse:
    if not is_valid_slug(page):
        raise NotFound()

    f = safe_path(f"{page}/{page}.md")
    p = await _page(f, page)

    return json({"page": p.json()})


async def _pages(directory: str) -> list[Page]:
    pages: list[Page] = []

    if not os.path.exists(directory):
        return pages

    for item in os.listdir(directory):
        item_path = os.path.join(directory, item)

        if os.path.isdir(item_path):
            pages.extend(await _pages(item_path))

        elif item.endswith(".md"):
            parent_dir_name = os.path.basename(directory)
            file_name_without_ext = item[:-3]

            if file_name_without_ext == parent_dir_name:
                pages.append(await _page(item_path, file_name_without_ext))

    def get_sort_key(page: Page):
        return page.updated or page.created or datetime.min

    pages.sort(key=get_sort_key, reverse=True)

    return pages


async def _page(path: str, slug: str) -> Page:
    if not cached_file_exists(path):
        raise NotFound()

    try:
        content = cached_file_read(path)
        post = frontmatter.loads(content)
        markdown_content = post.content

        page_slug: str = slug
        page_title: str = str(post.get("title", slug.replace("-", " ").title()))
        page_description: str | None = str(post.get("description", None))
        page_created: datetime | None = None
        page_updated: datetime | None = None
        page_tags: list[str] = []
        page_banner: str | None = None
        page_body: str = str(markdown_content)
        page_folder: str = os.path.basename(os.path.dirname(path))
        page_path: str = path
        page_type: str | None = None
        page_reading: list[ReadingItem] = []

        # Determine the topic based on path structure
        content_dir = get_content_dir()
        relative_path = path
        if relative_path.startswith(content_dir):
            relative_path = relative_path[len(content_dir) :]
            if relative_path.startswith("/"):
                relative_path = relative_path[1:]

        # Split path components
        path_parts = relative_path.split("/")

        # The topic is the first directory in the path, if it exists
        # and isn't just the folder containing the markdown file with the same name
        page_topic = None
        if len(path_parts) > 2:  # More than [folder]/[file].md
            first_dir = path_parts[0]
            if first_dir != page_slug:
                page_topic = first_dir.capitalize()

        _tags = post.get("tags", None)
        _created = post.get("created", None)
        _updated = post.get("updated", None)
        _banner = post.get("banner", None)
        _type = post.get("type", None)
        _reading = post.get("reading", None)

        if type(_tags) is list:
            page_tags = _tags

        if type(_created) is str:
            page_created = datetime.strptime(_created, "%Y-%m-%d")

        if type(_updated) is str:
            page_updated = datetime.strptime(_updated, "%Y-%m-%d")

        if type(_banner) is str:
            page_banner = _banner

        if type(_type) is str:
            page_type = _type

        if type(_reading) is list:
            for item in _reading:
                if (
                    type(item) is dict
                    and "title" in item
                    and "author" in item
                    and "url" in item
                ):
                    page_reading.append(
                        ReadingItem(
                            title=item["title"], author=item["author"], url=item["url"]
                        )
                    )

        return Page(
            slug=page_slug,
            title=page_title,
            body=page_body,
            path=page_path,
            description=page_description,
            created=page_created,
            updated=page_updated,
            tags=page_tags,
            banner=page_banner,
            folder=page_folder,
            topic=page_topic,
            type=page_type,
            reading=page_reading,
        )
    except Exception:
        raise NotFound()
