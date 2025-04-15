import datetime
import html
import os
import re
from functools import lru_cache

from fastapi import FastAPI, HTTPException, Response

from api._config import get_feeds_config, get_site_config
from api._content import _pages
from api._validation import get_content_dir

app = FastAPI()


def strip_markdown(text: str) -> str:
    """Strip markdown syntax from text (links, images, bold, italic, etc.)."""
    text = re.sub(r"!\[.*?\]\(.*?\)", "", text)  # Remove images
    text = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", text)  # Replace links with just the text
    text = re.sub(r"[*_]{2}(.*?)[*_]{2}", r"\1", text)  # Remove bold/italic markers
    text = re.sub(r"[*_](.*?)[*_]", r"\1", text)  # Remove bold/italic markers
    text = re.sub(r"#{1,6}\s+(.*?)$", r"\1", text, flags=re.MULTILINE)  # Remove headings
    text = re.sub(r"```.*?```", "", text, flags=re.DOTALL)  # Remove code blocks
    return text


@lru_cache(maxsize=1024)
def _get_feed(feed: str) -> Response:
    """Generate RSS feed for given slug."""
    feeds_config = get_feeds_config()
    site_config = get_site_config()

    base_url = site_config.get("url", "https://example.dev")
    site_title = site_config.get("title", "Some Site")

    feeds = feeds_config.get("feeds", {})

    matching_feed = None
    for feed_config in feeds:
        if feed_config.get("feed") == feed:
            matching_feed = feed_config
            break

    if not matching_feed:
        raise HTTPException(status_code=404, detail=f"Feed `{feed}` not found")

    feed_title = matching_feed.get("title", f"{site_title} - {feed.title()}")
    feed_description = matching_feed.get("description", f"Recent {feed} from {site_title}")
    feed_path = matching_feed.get("path", "")
    feed_types = matching_feed.get("types", "")

    # Get content from the specified path
    content_path = get_content_dir(feed_path)

    # Build RSS XML
    now = datetime.datetime.now().strftime("%a, %d %b %Y %H:%M:%S +0000")

    rss_items = []

    # Use _pages to get all pages in the directory
    if os.path.exists(content_path):
        try:
            pages = _pages(content_path)

            if feed_types:
                types = feed_types.split(",")
                pages = [p for p in pages if p.type in types]

            # Process each page
            for page in pages:
                try:
                    # Get relative path from content directory for URL
                    rel_path = os.path.relpath(page.path, get_content_dir())
                    # Remove .md extension
                    path = os.path.splitext(rel_path)[0]
                    # Convert to URL path
                    url_path = path.replace("\\", "/")

                    # Format date in RFC 822 format
                    pub_date = now
                    if page.created:
                        pub_date = page.created.strftime("%a, %d %b %Y %H:%M:%S +0000")

                    # Create excerpt
                    excerpt = page.description if page.description else strip_markdown(page.body[:500])

                    if not page.description and len(page.body) > 500:
                        excerpt += "..."

                    # Escape HTML entities
                    title_safe = html.escape(page.title)
                    excerpt_safe = html.escape(excerpt)
                    rss_items.append(f"""
        <item>
            <title>{title_safe}</title>
            <link>{base_url}/{url_path}</link>
            <guid>{base_url}/{url_path}</guid>
            <pubDate>{pub_date}</pubDate>
            <description>{excerpt_safe}</description>
        </item>
                    """)
                except Exception as e:
                    raise HTTPException(status_code=500, detail=f"Error processing {page.path}: {e}") from e
        except Exception as e:
            raise HTTPException(status_code=500, detail=f"Error loading pages from {content_path}: {e}") from e

    # Create RSS XML
    xml = f"""<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>{html.escape(feed_title)}</title>
        <link>{base_url}/{feed_path}</link>
        <description>{html.escape(feed_description)}</description>
        <language>en-us</language>
        <lastBuildDate>{now}</lastBuildDate>
        <atom:link href="{base_url}/api/rss/{feed}" rel="self" type="application/rss+xml" />
{"".join(rss_items)}
    </channel>
</rss>
"""

    return Response(xml, media_type="application/rss+xml")


@app.get("/api/feed")
def get_feed(feed: str | None = None) -> Response:
    """Get a specified feed."""
    if not feed:
        raise HTTPException(status_code=404, detail="Feed not found")

    return _get_feed(feed)
