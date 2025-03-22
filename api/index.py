from sanic import Sanic
from sanic.response import json
from sanic.request import Request

from content import content_bp
# from blueprints.test import bp as test_bp
# from api.sitemap import sitemap_bp
# from api.llms import llms_bp
# from api.rss import rss_bp

app = Sanic(
    name=__name__,
    strict_slashes=True,
)


@app.get("/api")
async def index(request: Request):
    return json({"name": "evansims.com", "version": "1.0.0", "status": "OK"})


@app.exception(Exception)
async def handle_exception(request: Request, exception: Exception):
    status_code = getattr(exception, "status_code", 500)
    error_message = str(exception) or "An unexpected error occurred"

    return json(
        {"error": True, "message": error_message, "status": status_code},
        status=status_code,
    )

# app.blueprint(content_bp)
app.blueprint(content_bp)
# app.blueprint(sitemap_bp)
# app.blueprint(llms_bp)
# app.blueprint(rss_bp)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5328, access_log=False, dev=True)
