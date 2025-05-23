import asyncio
import io
import re
from functools import lru_cache
from pathlib import Path

from fastapi import FastAPI, HTTPException
from fastapi.responses import StreamingResponse
from PIL import Image

# Configuration
IMAGE_DIR = Path("public/images")
CACHE_TIME = 86400 * 30  # 30 days
ALLOWED_FORMATS = {"webp", "avif", "jpeg"}
VALID_SLUG = re.compile(r"^[a-z0-9-]{1,64}$")

app = FastAPI()


@lru_cache(maxsize=1024)
def _get_image(slug: str, w: int = 0, q: int = 85, f: str = "webp") -> StreamingResponse:
    """Get an optimized image from the cache or generate a new one."""
    # Security validation
    if not VALID_SLUG.match(slug):
        raise HTTPException(status_code=400, detail="Invalid image request")

    # Validate format
    if f not in ALLOWED_FORMATS:
        f = "webp"

    # Find original image
    original_path = next(IMAGE_DIR.glob(f"{slug}.*"), None)

    if not original_path or not original_path.is_file():
        raise HTTPException(status_code=404, detail="Image not found")

    # Optimization pipeline
    lock = app.ctx.locks.setdefault(slug, asyncio.Lock())

    with lock:
        optimized_path = IMAGE_DIR / f"{slug}_{w}w_{q}q.{f}"

        if not optimized_path.exists():
            with Image.open(original_path) as img:
                if w and 0 < w < img.width:
                    ratio = w / img.width
                    height = int(img.height * ratio)
                    img = img.resize((w, height))

                buffer = io.BytesIO()
                img.save(buffer, format=f, quality=q, optimize=True)
                buffer.seek(0)
                optimized_path.write_bytes(buffer.getvalue())

    # Vercel-specific optimizations
    headers = {
        "Cache-Control": f"public, max-age={CACHE_TIME}, immutable",
        "CDN-Cache-Control": f"max-age={CACHE_TIME}",
        "Vercel-CDN-Cache-Control": f"max-age={CACHE_TIME}",
    }

    return StreamingResponse(optimized_path.open("rb"), headers=headers, media_type=f"image/{f}")


@app.get("/api/images")
def get_image(slug: str, w: int = 0, q: int = 85, f: str = "webp") -> StreamingResponse:
    """Get an optimized image from the cache or generate a new one."""
    return _get_image(slug, w, q, f)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
