from flask import Flask, jsonify, abort
import os

app = Flask(__name__)
CONTENT_DIR = os.path.join(os.getcwd(), 'content')

@app.route('/api/content/<slug>', methods=['GET'])
def get_content(slug):
    file_path = os.path.join(CONTENT_DIR, f"{slug}.md")

    if not os.path.exists(file_path):
        abort(404)

    try:
        with open(file_path, 'r') as f:
            return jsonify({
                'title': slug.replace('-', ' ').title(),
                'body': f.read()
            })
    except Exception:
        abort(500, description="Error reading content")