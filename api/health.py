from fastapi import FastAPI

app = FastAPI()


@app.get("/api/health")
async def health() -> dict:
    """Health check."""
    return {"status": "ok"}


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="127.0.0.1", port=5328, reload=True)
