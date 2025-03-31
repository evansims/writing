from fastapi import FastAPI

app = FastAPI()


@app.get("/api/")
async def health() -> dict:
    """Health check."""
    return {"status": "ok"}
