from sanic import Blueprint
from sanic.response import json
from sanic.exceptions import NotFound
import frontmatter

from api.common.validation import safe_path
from api.common.filesystem import cached_file_exists, cached_file_read

content_bp = Blueprint('content_routes', url_prefix='/api/content')

@content_bp.route('/<slug>', methods=['GET'])
async def get_content(request, slug):
    file_path = safe_path(slug)

    if not cached_file_exists(file_path):
        raise NotFound(f"Content with slug '{slug}' not found")

    try:
        content_text = cached_file_read(file_path)
        post = frontmatter.loads(content_text)

        return json({
            'title': post.get('title', slug.replace('-', ' ').title()),
            'description': post.get('description', ''),
            'created': post.get('created', ''),
            'updated': post.get('updated', ''),
            'tags': post.get('tags', []),
            'banner': post.get('banner', ''),
            'content': post.content
        })
    except Exception as e:
        raise NotFound(f"Error reading content: {str(e)}")
