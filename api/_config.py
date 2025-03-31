import os

import yaml


def get_site_config() -> dict:
    """Get the site config."""
    try:
        config_path = os.path.join(os.getcwd(), "configuration", "site.yml")
        with open(config_path) as f:
            return yaml.safe_load(f)
    except Exception:
        return {"url": "https://evansims.com", "title": "Evan Sims"}


def get_rss_config() -> dict:
    """Get the RSS config."""
    try:
        config_path = os.path.join(os.getcwd(), "configuration", "rss.yml")
        with open(config_path) as f:
            return yaml.safe_load(f)
    except Exception:
        return {}


def get_llms_config() -> dict:
    """Get the llms config."""
    try:
        config_path = os.path.join(os.getcwd(), "configuration", "llms.yml")
        with open(config_path) as f:
            return yaml.safe_load(f)
    except Exception:
        return {}
