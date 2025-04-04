# Writing

A personal writing collection exploring health,creativity, engineering, focus, mindset, strategy, and tools.

## Quick Start

### Configuration

1. Get an API key from [Eleven Labs](https://elevenlabs.io/).
2. Create or update the `.env` file in the project root with your API key:

```bash
ELEVENLABS_API_KEY=your_api_key_here
```

### Installing Tools

You can install the required tools using [asdf](https://asdf.run/) and the provided [.tool-versions](.tool-versions) file.

```bash
asdf install
```

### Running Backend

Create a virtual environment and install dependencies:

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

Run the backend:

```bash
npm run api
```

### Running Frontend

```bash
vercel dev
```

### Content Management CLI

```bash
# Show available commands
./write --help

# Launch interactive mode
./write interactive
```

## Structure

- `api/` - Python-based backend APIs
- `app/` - NextJS-based frontend application
- `components/` - React components
- `configuration/` - YAML configuration files
- `content/` - Markdown content
- `docs/` - Documentation
- `hooks/` - Custom React hooks
- `lib/` - Frontend utilities
- `templates/` - Article templates
- `tools/` - Content management tools
- `config.yaml` - Configuration

## License

This project is licensed under the Creative Commons Attribution 4.0 International License - see the [LICENSE](LICENSE) file for details.
