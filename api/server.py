from fastapi import FastAPI

from api.audio import app as audio_routes
from api.content import app as content_routes
from api.health import app as health_routes
from api.llms import app as llms_routes

app = FastAPI()

app.include_router(health_routes.router)
app.include_router(audio_routes.router)
app.include_router(content_routes.router)
app.include_router(llms_routes.router)

if __name__ == "__main__":
    import uvicorn

    uvicorn.run("server:app", host="127.0.0.1", port=5328, reload=True)
