from sanic import Sanic, response
from sanic.response import file_stream
from pathlib import Path
import re
from PIL import Image
import io
import asyncio
app = Sanic("ImageServer")

# Configuration
IMAGE_DIR = Path("public/images")
CACHE_TIME = 86400 * 30  # 30 days
ALLOWED_FORMATS = {"webp", "avif", "jpeg"}
VALID_SLUG = re.compile(r"^[a-z0-9-]{1,64}$")

@app.get("/images/<slug:str>")
async def serve_image(request, slug: str):
    # Security validation
    if not VALID_SLUG.match(slug):
        return response.json({"error": "Invalid image request"}, status=400)

    # Extract optimization params
    width = int(request.args.get("w", 0))
    quality = int(request.args.get("q", 85))
    format = request.args.get("f", "webp")

    # Validate format
    if format not in ALLOWED_FORMATS:
        format = "webp"

    # Find original image
    original_path = next(IMAGE_DIR.glob(f"{slug}.*"), None)
    if not original_path or not original_path.is_file():
        return response.json({"error": "Image not found"}, status=404)

    # Optimization pipeline
    async with request.app.ctx.locks.setdefault(slug, asyncio.Lock()):
        async with request.app.ctx.locks[slug]:
            optimized_path = IMAGE_DIR / f"{slug}_{width}w_{quality}q.{format}"

            if not optimized_path.exists():
                with Image.open(original_path) as img:
                    if width and 0 < width < img.width:
                        ratio = width / img.width
                        height = int(img.height * ratio)
                        img = img.resize((width, height))

                    buffer = io.BytesIO()
                    img.save(buffer, format=format, quality=quality, optimize=True)
                    buffer.seek(0)
                    optimized_path.write_bytes(buffer.getvalue())

    # Vercel-specific optimizations
    headers = {
        "Cache-Control": f"public, max-age={CACHE_TIME}, immutable",
        "CDN-Cache-Control": f"max-age={CACHE_TIME}",
        "Vercel-CDN-Cache-Control": f"max-age={CACHE_TIME}"
    }

    return await file_stream(
        str(optimized_path),
        headers=headers,
        mime_type=f"image/{format}"
    )
