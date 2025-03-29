import json as json_lib
import os
from datetime import datetime

from sanic import Sanic
from sanic.request import Request
from sanic.response import json

from .audio import audio_bp
from .content import content_bp

# from api.sitemap import sitemap_bp
# from api.llms import llms_bp
# from api.rss import rss_bp

API_DEBUG_LOGGING = True

app = Sanic(
    name="evansims",
    strict_slashes=True,
    env_prefix="EVANSIMS_",
)


# Request logging middleware
@app.middleware("request")
async def log_request(request):
    if API_DEBUG_LOGGING:
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S.%f")[:-3]
        print(f"\n[{timestamp}] 🔷 REQUEST: {request.method} {request.path}")
        print(f"  Headers: {json_lib.dumps(dict(request.headers), indent=2)}")
        if request.body:
            try:
                body = request.json
                print(f"  Body: {json_lib.dumps(body, indent=2)}")
            except Exception:
                if len(request.body) > 1000:
                    print(f"  Body: [Binary data of length {len(request.body)}]")
                else:
                    print(f"  Body: {request.body}")
        print(f"  Query Params: {json_lib.dumps(dict(request.query_args), indent=2)}")


# Response logging middleware
@app.middleware("response")
async def log_response(request, response):
    if API_DEBUG_LOGGING:
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S.%f")[:-3]
        print(f"[{timestamp}] 🔶 RESPONSE: {response.status}")
        print(f"  Headers: {json_lib.dumps(dict(response.headers), indent=2)}")
        if hasattr(response, "body") and response.body:
            body_str = response.body.decode("utf-8", errors="replace")
            if len(body_str) > 1000:
                print(f"  Body: [Data of length {len(body_str)}]")
                try:
                    data = json_lib.loads(body_str)
                    keys = ", ".join(data.keys())
                    print(f"  Content keys: {keys}")
                except Exception:
                    pass
            else:
                try:
                    data = json_lib.loads(body_str)
                    print(f"  Body: {json_lib.dumps(data, indent=2)}")
                except Exception:
                    print(f"  Body: {body_str[:500]}...")
        print("\n" + "-" * 80)


@app.get("/api")
async def index(request: Request):
    return json({"name": "evansims.com", "version": "1.0.0", "status": "OK"})


@app.get("/api/debug")
async def debug_info(request: Request):
    """Debug endpoint to test logging functionality."""
    return json(
        {
            "debug": {
                "logging_enabled": API_DEBUG_LOGGING,
                "debug_mode": os.getenv("API_DEBUG", "false").lower() == "true",
                "timestamp": datetime.now().isoformat(),
                "request_headers": dict(request.headers),
                "query_params": dict(request.query_args),
            }
        }
    )


@app.exception(Exception)
async def handle_exception(request: Request, exception: Exception):
    status_code = getattr(exception, "status_code", 500)
    error_message = str(exception) or "An unexpected error occurred"

    if API_DEBUG_LOGGING:
        import traceback

        print(f"\n❌ EXCEPTION: {status_code} - {error_message}")
        print(traceback.format_exc())

    return json(
        {"error": True, "message": error_message, "status": status_code},
        status=status_code,
    )


app.blueprint(content_bp)
app.blueprint(audio_bp)
# app.blueprint(sitemap_bp)
# app.blueprint(llms_bp)
# app.blueprint(rss_bp)


def handler(request, response):
    return app.response_class(
        app.router.get_supported_methods(request), request=request, response=response
    )


if __name__ == "__main__":
    app.run()
