import yaml

from api._filesystem import cached_file_read, get_config_path


def get_site_config() -> dict:
    """Get the site config."""
    try:
        config_path = get_config_path("site.yml")
        config_content = cached_file_read(config_path)
        return yaml.safe_load(config_content)
    except Exception:
        raise Exception(f"Failed to load site config: {config_path}") from None


def get_feeds_config() -> dict:
    """Get the feeds config."""
    try:
        config_path = get_config_path("feeds.yml")
        config_content = cached_file_read(config_path)
        return yaml.safe_load(config_content)
    except Exception:
        return {}


def get_llms_config() -> dict:
    """Get the llms config."""
    try:
        config_path = get_config_path("llms.yml")
        config_content = cached_file_read(config_path)
        return yaml.safe_load(config_content)
    except Exception:
        return {}
