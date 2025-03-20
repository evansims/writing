import os

def validate_slug(slug: str) -> bool:
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789-")
    return all(c in allowed for c in slug.lower()) and 3 <= len(slug) <= 64

def safe_path(slug: str) -> str:
    if not validate_slug(slug):
        abort(400, description="Invalid slug format")

    base_path = os.path.abspath(CONTENT_DIR)
    target_path = os.path.abspath(os.path.join(base_path, f"{slug}.md"))

    if os.path.commonpath([base_path, target_path]) != base_path:
        abort(400, description="Invalid path traversal attempt")

    return target_path