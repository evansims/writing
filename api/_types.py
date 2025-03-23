from dataclasses import dataclass, field
from datetime import datetime


@dataclass
class ReadingItem:
    title: str
    author: str
    url: str

    def json(self) -> dict:
        return {"title": self.title, "author": self.author, "url": self.url}


@dataclass
class Page:
    slug: str
    title: str
    body: str
    path: str
    description: str | None = None
    created: datetime | None = None
    updated: datetime | None = None
    tags: list[str] = field(default_factory=list)
    banner: str | None = None
    folder: str | None = None
    topic: str | None = None
    type: str | None = None
    reading: list[ReadingItem] = field(default_factory=list)

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
            "url": f"/{_url}",
            "topic": self.topic,
            "type": self.type,
            "body": self.body,
            "reading": [item.json() for item in self.reading] if self.reading else None,
        }


@dataclass
class Section:
    name: str
    path: str
    pages: list[Page]
