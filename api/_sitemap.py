from sanic import Blueprint
from sanic.response import text
import os
import glob
import yaml
import datetime

from ._validation import get_content_path

sitemap_bp = Blueprint("sitemap_routes", url_prefix="/api")


def get_site_config():
    try:
        config_path = os.path.join(os.getcwd(), "configuration", "site.yml")
        with open(config_path, "r") as f:
            return yaml.safe_load(f)
    except Exception:
        return {"url": "https://evansims.com"}  # Default fallback


@sitemap_bp.route("/sitemap.xml", methods=["GET"])
async def sitemap(request):
    site_config = get_site_config()
    base_url = site_config.get("url", "https://evansims.com")

    # Find all content files recursively
    content_files = glob.glob(f"{get_content_path()}/**/*.md", recursive=True)

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
        rel_path = os.path.relpath(content_file, get_content_path())
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

    return text(xml_content, content_type="application/xml")
