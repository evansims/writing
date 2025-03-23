from dataclasses import dataclass
from datetime import datetime


@dataclass
class Page:
    slug: str
    title: str
    description: str | None
    created: datetime | None
    updated: datetime | None
    tags: list[str]
    banner: str | None
    body: str
    folder: str | None
    path: str
    topic: str | None = None

    def json(self) -> dict:
        from _filesystem import get_content_dir

        base_path = get_content_dir()
        _url = self.path
        if _url.startswith(base_path):
            _url = _url[len(base_path) :]
            if _url.startswith("/"):
                _url = _url[1:]

        _url = _url.split("/")
        _url.pop()
        _url = "/".join(_url)

        return {
            "slug": self.slug,
            "title": self.title,
            "description": self.description,
            "created": self.created.isoformat() if self.created else None,
            "updated": self.updated.isoformat() if self.updated else None,
            "tags": self.tags,
            "banner": self.banner,
            "body": self.body,
            "url": f"/{_url}",
            "topic": self.topic,
        }


@dataclass
class Section:
    name: str
    path: str
    pages: list[Page]
