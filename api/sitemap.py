import datetime
import glob
import os

from fastapi import FastAPI
from fastapi.responses import StreamingResponse

from api._config import get_site_config
from api._validation import get_content_dir

app = FastAPI()


@app.get("/api/sitemap")
async def sitemap() -> StreamingResponse:
    """Generate sitemap XML."""
    site_config = get_site_config()
    base_url = site_config.get("url", "https://evansims.com")

    # Find all content files recursively
    content_files = glob.glob(f"{get_content_dir()}/**/*.md", recursive=True)

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
    for content_file in content_files:
        # Get relative path from content directory
        rel_path = os.path.relpath(content_file, get_content_dir())
        # Remove .md extension
        path = os.path.splitext(rel_path)[0]
        # Convert to URL path
        url_path = path.replace("\\", "/")

        # Get file modification time
        try:
            mtime = os.path.getmtime(content_file)
            lastmod = datetime.datetime.fromtimestamp(mtime).strftime("%Y-%m-%d")
        except Exception:
            lastmod = today

        sitemap_items.append(f"""
    <url>
        <loc>{base_url}/{url_path}</loc>
        <lastmod>{lastmod}</lastmod>
        <changefreq>weekly</changefreq>
        <priority>0.8</priority>
    </url>
    """)

    # Create XML content
    xml_content = f"""<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{"".join(sitemap_items)}
</urlset>
"""

    return StreamingResponse(xml_content, media_type="application/xml")


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
