from sanic import Blueprint
from sanic.response import text
from sanic.exceptions import NotFound
import os
import glob
import datetime
import frontmatter
import yaml
import html
import re

from api.common.validation import CONTENT_DIR

rss_bp = Blueprint('rss_routes', url_prefix='/api/rss')

def get_rss_config():
    try:
        config_path = os.path.join(os.getcwd(), 'configuration', 'rss.yml')
        with open(config_path, 'r') as f:
            return yaml.safe_load(f)
    except Exception:
        return {}  # Default empty config

def get_site_config():
    try:
        config_path = os.path.join(os.getcwd(), 'configuration', 'site.yml')
        with open(config_path, 'r') as f:
            return yaml.safe_load(f)
    except Exception:
        return {'url': 'https://evansims.com', 'title': 'Evan Sims'}  # Default fallback

def strip_markdown(text):
    # Basic markdown stripping (links, images, bold, italic, etc.)
    text = re.sub(r'!\[.*?\]\(.*?\)', '', text)  # Remove images
    text = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', text)  # Replace links with just the text
    text = re.sub(r'[*_]{2}(.*?)[*_]{2}', r'\1', text)  # Remove bold/italic markers
    text = re.sub(r'[*_](.*?)[*_]', r'\1', text)  # Remove bold/italic markers
    text = re.sub(r'#{1,6}\s+(.*?)$', r'\1', text, flags=re.MULTILINE)  # Remove headings
    text = re.sub(r'```.*?```', '', text, flags=re.DOTALL)  # Remove code blocks
    return text

@rss_bp.route('/<slug>', methods=['GET'])
async def feed(request, slug):
    """Generate RSS feed for given slug."""
    rss_config = get_rss_config()
    site_config = get_site_config()

    base_url = site_config.get('url', 'https://evansims.com')
    site_title = site_config.get('title', 'Evan Sims')

    # Validate that this is a configured feed
    feeds = rss_config.get('feeds', {})
    if slug not in feeds:
        raise NotFound(f"RSS feed '{slug}' not found")

    feed_config = feeds[slug]
    feed_title = feed_config.get('title', f"{site_title} - {slug.title()}")
    feed_description = feed_config.get('description', f"Recent {slug} from {site_title}")
    feed_path = feed_config.get('path', slug)

    # Get content from the specified path
    content_path = os.path.join(CONTENT_DIR, feed_path)
    content_files = []

    if os.path.exists(content_path):
        # Find all markdown files in this directory (and subdirectories if recursive is true)
        pattern = f'{content_path}/**/*.md' if feed_config.get('recursive', True) else f'{content_path}/*.md'
        content_files = glob.glob(pattern, recursive=True)

    # Build RSS XML
    now = datetime.datetime.now().strftime('%a, %d %b %Y %H:%M:%S +0000')

    rss_items = []
    for content_file in sorted(content_files, key=os.path.getmtime, reverse=True):
        try:
            with open(content_file, 'r') as f:
                post = frontmatter.load(f)

                # Get relative path from content directory for URL
                rel_path = os.path.relpath(content_file, CONTENT_DIR)
                # Remove .md extension
                path = os.path.splitext(rel_path)[0]
                # Convert to URL path
                url_path = path.replace('\\', '/')

                title = post.get('title', os.path.basename(path).replace('-', ' ').title())

                # Format date in RFC 822 format
                pub_date = post.get('created', '')
                if pub_date:
                    try:
                        dt = datetime.datetime.strptime(pub_date, '%Y-%m-%d')
                        pub_date = dt.strftime('%a, %d %b %Y %H:%M:%S +0000')
                    except:
                        pub_date = now
                else:
                    pub_date = now

                # Create excerpt
                description = post.get('description', '')
                excerpt = description if description else strip_markdown(post.content[:500])
                if not description and len(post.content) > 500:
                    excerpt += "..."

                # Escape HTML entities
                title_safe = html.escape(title)
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
            print(f"Error processing {content_file}: {e}")

    # Create RSS XML
    rss_xml = f"""<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>{html.escape(feed_title)}</title>
        <link>{base_url}/{feed_path}</link>
        <description>{html.escape(feed_description)}</description>
        <language>en-us</language>
        <lastBuildDate>{now}</lastBuildDate>
        <atom:link href="{base_url}/api/rss/{slug}" rel="self" type="application/rss+xml" />
{''.join(rss_items)}
    </channel>
</rss>
"""

    return text(rss_xml, content_type="application/xml")
