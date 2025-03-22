from sanic import Blueprint, Request
from sanic.response import json

bp = Blueprint('test_routes', url_prefix='/api/test')

@bp.get('/')
async def hello_world(request: Request):
    return json({
        'test': "wakka wakka"
    })