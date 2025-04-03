from fastapi import FastAPI

from audio import app as audio_routes
from content import app as content_routes
from health import app as health_routes
from llms import app as llms_routes

app = FastAPI()

app.include_router(health_routes.router)
app.include_router(audio_routes.router)
app.include_router(content_routes.router)
app.include_router(llms_routes.router)

if __name__ == "__main__":
    import uvicorn

    uvicorn.run("server:app", host="127.0.0.1", port=5328, reload=True)
