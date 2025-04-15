import datetime
from functools import lru_cache

from fastapi import FastAPI, Response

from api._config import get_site_config
from api._content import _pages
from api._validation import get_content_dir

app = FastAPI()


@lru_cache(maxsize=1024)
def _sitemap() -> Response:
    """Generate sitemap XML."""
    site_config = get_site_config()
    base_url = site_config.get("url", "https://example.dev")

    # Get all content pages
    all_pages = _pages(get_content_dir())

    # Generate sitemap XML
    sitemap_items = []
    today = datetime.datetime.now().strftime("%Y-%m-%d")

    # Add homepage
    sitemap_items.append(f"""
    <url>
        <loc>{base_url}</loc>
        <lastmod>{today}</lastmod>
        <changefreq>daily</changefreq>
        <priority>1.0</priority>
    </url>
    """)

    # Add content pages
    for page in all_pages:
        url_path = page.slug
        lastmod_dt = page.updated or page.created
        lastmod = lastmod_dt.strftime("%Y-%m-%d") if lastmod_dt else today

        sitemap_items.append(f"""
    <url>
        <loc>{base_url}/{url_path}</loc>
        <lastmod>{lastmod}</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.8</priority>
    </url>
    """)

    # Create XML content
    xml = f"""<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{"".join(sitemap_items)}
</urlset>
"""

    # Ensure correct media type for sitemap
    return Response(xml, media_type="application/xml")


@app.get("/api/sitemap")
def sitemap() -> Response:
    """Generate sitemap XML."""
    return _sitemap()
