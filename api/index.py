from sanic import Sanic
from sanic.response import json

# Import all blueprints - directly from files since they're not modules
from api.content import content_bp
from api.sitemap import sitemap_bp
from api.llms import llms_bp
from api.rss import rss_bp

app = Sanic("evansims_api")

# Register all blueprints
app.blueprint(content_bp)
app.blueprint(sitemap_bp)
app.blueprint(llms_bp)
app.blueprint(rss_bp)

@app.route('/')
async def index(request):
    return json({
        'name': 'Evan Sims API',
        'version': '1.0.0',
        'status': 'ok'
    })

# For local development
if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000, debug=True)
