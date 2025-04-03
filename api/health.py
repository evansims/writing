from fastapi import FastAPI
from fastapi.responses import StreamingResponse

app = FastAPI()


@app.get("/api/health")
async def health() -> StreamingResponse:
    """Health check."""
    return StreamingResponse({"status": "ok"})


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
