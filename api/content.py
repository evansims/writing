from datetime import datetime
import os
from sanic import Blueprint, Request
from sanic.response import json, JSONResponse
from sanic.exceptions import NotFound
import frontmatter
import mistune

from _validation import safe_path, is_valid_slug, is_valid_path
from _filesystem import cached_file_exists, cached_file_read, get_content_dir
from _types import Page

content_bp = Blueprint("content_routes", url_prefix="/api/content")


@content_bp.get("/")
async def list_pages(request: Request) -> JSONResponse:
    ps = await _pages(get_content_dir())

    return json(
        {
            "pages": [p.json() for p in ps],
        }
    )


@content_bp.get("/<folder:path>/")
async def list_nested_pages(request: Request, folder) -> JSONResponse:
    if not is_valid_path(folder):
        raise NotFound()

    try:
        f = safe_path(folder)
    except Exception:
        return json([])

    ps = await _pages(f)

    return json(
        {
            "pages": [p.json() for p in ps],
        }
    )


@content_bp.get("/<folder:path>/<page:slug>")
async def get_nested_content(
    request: Request,
    folder: str,
    page: str,
) -> JSONResponse:
    if not is_valid_path(folder) or not is_valid_slug(page):
        raise NotFound()

    f = safe_path(f"{folder}/{page}/{page}.md")
    p = await _page(f, page)

    return json({"page": p.json()})


@content_bp.get("/<page:slug>")
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
        print(item)
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
        html = mistune.markdown(post.content)

        page_title: str = str(post.get("title", slug.replace("-", " ").title()))
        page_description: str | None = str(post.get("description", None))
        page_created: datetime | None = None
        page_updated: datetime | None = None
        page_tags: list[str] = []
        page_banner: str | None = str(post.get("banner", None))
        page_body: str = str(html)
        page_slug: str = slug
        page_folder: str = os.path.basename(os.path.dirname(path))
        page_path: str = path

        _tags = post.get("tags", None)
        _created = post.get("created", None)
        _updated = post.get("updated", None)

        if type(_tags) is list:
            page_tags = _tags

        if type(_created) is str:
            page_created = datetime.strptime(_created, "%Y-%m-%d")

        if type(_updated) is str:
            page_updated = datetime.strptime(_updated, "%Y-%m-%d")

        return Page(
            page_slug,
            page_title,
            page_description,
            page_created,
            page_updated,
            page_tags,
            page_banner,
            page_body,
            page_folder,
            page_path,
        )
    except Exception:
        raise NotFound()
