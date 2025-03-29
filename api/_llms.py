from sanic import Blueprint
from sanic.response import text
import os
import glob
import frontmatter
import yaml

from ._validation import get_content_path

llms_bp = Blueprint("llms_routes", url_prefix="/api")


def get_llms_config():
    try:
        config_path = os.path.join(os.getcwd(), "configuration", "llms.yml")
        with open(config_path, "r") as f:
            return yaml.safe_load(f)
    except Exception:
        return {}  # Default empty config


@llms_bp.route("/llms.txt", methods=["GET"])
async def llms_txt(request):
    """Serves a condensed version of content for LLMs."""
    content_files = glob.glob(f"{get_content_path()}/**/*.md", recursive=True)
    config = get_llms_config()

    # Get max entries from config, default to 100
    max_entries = config.get("max_entries", 100)
    max_summary_length = config.get("max_summary_length", 500)

    entries = []

    for content_file in sorted(content_files, key=os.path.getmtime, reverse=True)[
        :max_entries
    ]:
        try:
            with open(content_file, "r") as f:
                post = frontmatter.load(f)

                # Get relative path from content directory for URL
                rel_path = os.path.relpath(content_file, get_content_path())
                # Remove .md extension
                path = os.path.splitext(rel_path)[0]
                # Convert to URL path
                url_path = path.replace("\\", "/")

                title = post.get(
                    "title", os.path.basename(path).replace("-", " ").title()
                )
                description = post.get("description", "")
                created = post.get("created", "")

                # Add summary of the content
                content_summary = post.content[:max_summary_length]
                if len(post.content) > max_summary_length:
                    content_summary += "..."

                entries.append(
                    f"URL: /{url_path}\nTitle: {title}\nDate: {created}\n\n{description}\n\n{content_summary}\n\n---\n"
                )
        except Exception as e:
            print(f"Error processing {content_file}: {e}")

    return text("".join(entries), content_type="text/plain; charset=utf-8")


@llms_bp.route("/llms-full.txt", methods=["GET"])
async def llms_full_txt(request):
    """Serves a full version of content for LLMs."""
    content_files = glob.glob(f"{get_content_path()}/**/*.md", recursive=True)
    config = get_llms_config()

    # Get max entries from config, default to 50 for full content (to avoid huge responses)
    max_entries = config.get("max_full_entries", 50)

    entries = []

    for content_file in sorted(content_files, key=os.path.getmtime, reverse=True)[
        :max_entries
    ]:
        try:
            with open(content_file, "r") as f:
                post = frontmatter.load(f)

                # Get relative path from content directory for URL
                rel_path = os.path.relpath(content_file, get_content_path())
                # Remove .md extension
                path = os.path.splitext(rel_path)[0]
                # Convert to URL path
                url_path = path.replace("\\", "/")

                title = post.get(
                    "title", os.path.basename(path).replace("-", " ").title()
                )
                description = post.get("description", "")
                created = post.get("created", "")

                entries.append(
                    f"URL: /{url_path}\nTitle: {title}\nDate: {created}\n\n{description}\n\n{post.content}\n\n---\n"
                )
        except Exception as e:
            print(f"Error processing {content_file}: {e}")

    return text("".join(entries), content_type="text/plain; charset=utf-8")
