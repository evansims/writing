import datetime
import html
import os
import re
from functools import lru_cache

from fastapi import FastAPI, HTTPException, Response

from api._config import get_feeds_config, get_site_config
from api._content import _pages
from api._validation import get_content_dir
from api.audio import generate_or_get_full_audio, get_audio_dir, get_audio_path, split_content_into_chunks

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


# Cache the XML content, not the coroutine
@lru_cache(maxsize=1024)
def _get_feed_cached(feed: str) -> str:
    """Return cached feed XML content."""
    return ""  # Just a placeholder, this will be replaced by actual content


async def _generate_feed(feed: str) -> Response:
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

    # Check if this is a podcast feed (has audio flag)
    is_podcast = matching_feed.get("audio", False)

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

                    # Default item template
                    item_xml = f"""
        <item>
            <title>{title_safe}</title>
            <link>{base_url}/{url_path}</link>
            <guid>{base_url}/{url_path}</guid>
            <pubDate>{pub_date}</pubDate>
            <description>{excerpt_safe}</description>
        """

                    # Add podcast-specific elements if this is a podcast feed
                    if is_podcast:
                        try:
                            # Get audio file URL for the full audio (using format=mp3 parameter)
                            # For paths like "folder/filename", convert to "/api/audio/folder/filename/filename?format=mp3"
                            if "/" in url_path:
                                parts = url_path.split("/")
                                if len(parts) == 2:
                                    path = parts[0]
                                    slug = parts[1]
                                    # Use a format that's less ambiguous for FastAPI routing
                                    audio_file_url = f"{base_url}/api/audio/{path}/{slug}/{slug}?format=mp3"
                                else:
                                    # If not a standard folder/filename pattern, use the generic format
                                    audio_file_url = f"{base_url}/api/audio/{url_path}?format=mp3"
                            else:
                                # Simple slug case
                                audio_file_url = f"{base_url}/api/audio/{url_path}?format=mp3"

                            # Generate or get the full audio file to ensure it exists and get its size
                            audio_file_path, audio_size = await generate_or_get_full_audio(page.path, page.slug)

                            # Calculate approximate duration (assuming ~1MB per minute at 128kbps)
                            duration_seconds = int((audio_size / 1024 / 128) * 8)  # Size in KB / 128Kbps * 8 bits
                            minutes = duration_seconds // 60
                            seconds = duration_seconds % 60
                            duration = f"{minutes:02d}:{seconds:02d}"

                            # Add podcast elements
                            item_xml += f"""
            <enclosure url="{audio_file_url}" length="{audio_size}" type="audio/mpeg"/>
            <itunes:title>{title_safe}</itunes:title>
            <itunes:subtitle>{html.escape(page.description or "")}</itunes:subtitle>
            <itunes:summary>{excerpt_safe}</itunes:summary>
            <itunes:explicit>false</itunes:explicit>
            <itunes:duration>{duration}</itunes:duration>
            """
                        except Exception as audio_error:
                            print(f"Error getting audio for {page.path}: {audio_error}")

                    # Close the item tag
                    item_xml += """
        </item>"""

                    rss_items.append(item_xml)
                except Exception as e:
                    raise HTTPException(status_code=500, detail=f"Error processing {page.path}: {e}") from e
        except Exception as e:
            raise HTTPException(status_code=500, detail=f"Error loading pages from {content_path}: {e}") from e

    # Add podcast namespaces if needed
    podcast_ns = ""
    if is_podcast:
        podcast_ns = ' xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" xmlns:content="http://purl.org/rss/1.0/modules/content/"'

    # Create RSS XML
    xml = f"""<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom"{podcast_ns}>
    <channel>
        <title>{html.escape(feed_title)}</title>
        <link>{base_url}/{feed_path}</link>
        <description>{html.escape(feed_description)}</description>
        <language>en-us</language>
        <lastBuildDate>{now}</lastBuildDate>
        <atom:link href="{base_url}/api/feed?feed={feed}" rel="self" type="application/rss+xml" />"""

    # Add podcast channel elements if needed
    if is_podcast:
        author = site_config.get("author", "Evan Sims")
        xml += f"""
        <itunes:author>{html.escape(author)}</itunes:author>
        <itunes:owner>
            <itunes:name>{html.escape(author)}</itunes:name>
            <itunes:email>{site_config.get("email", "podcast@example.com")}</itunes:email>
        </itunes:owner>
        <itunes:image href="{base_url}/images/podcast-cover.jpg" />
        <itunes:category text="Technology" />
        <itunes:explicit>false</itunes:explicit>"""

    # Add items and close channel/rss tags
    xml += f"""
{"".join(rss_items)}
    </channel>
</rss>
"""

    # Update cache with the generated XML
    _get_feed_cached.cache_clear()  # Clear previous cache for this feed
    _get_feed_cached.__wrapped__.__dict__[feed] = xml  # Set new cache value

    return Response(xml, media_type="application/rss+xml")


@app.get("/api/feed")
async def get_feed(feed: str | None = None) -> Response:
    """Get a specified feed."""
    if not feed:
        raise HTTPException(status_code=404, detail="Feed not found")

    # Check if we have a cached version
    cached_xml = _get_feed_cached(feed)
    if cached_xml:
        return Response(cached_xml, media_type="application/rss+xml")

    # Generate a fresh feed
    return await _generate_feed(feed)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
