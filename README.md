# Writing

A personal writing collection exploring creativity, engineering, focus, mindset, strategy, and tools.

## Quick Start

```bash
# Show available commands
./write --help

# Launch interactive mode
./write interactive
```

## Structure

- `content/` - Articles by topic
- `docs/` - Documentation
- `templates/` - Article templates
- `tools/` - Content management tools
- `config.yaml` - Configuration

## Text-to-Speech Feature

This project includes text-to-speech functionality powered by the Eleven Labs API (using SDK version 1.54.0). To enable this feature:

1. Get an API key from [Eleven Labs](https://elevenlabs.io/).
2. Create or update the `.env` file in the project root with your API key:

```bash
ELEVENLABS_API_KEY=your_api_key_here
```

3. Start the backend API:

```bash
cd api
python -m sanic index:app
```

4. Run the frontend application:

```bash
npm run dev
```

5. Visit any content page to see the audio player.

### Features

- **Paragraph-by-paragraph playback**: Audio is generated and played for each paragraph separately
- **Text highlighting**: The current paragraph being read is highlighted
- **Playback controls**: Play/pause, skip forward/backward, volume control, and speed adjustment
- **Voice**: Uses Will's voice from Eleven Labs for all audio
- **Caching**: Audio files are cached for improved performance and reduced API usage

### Troubleshooting

If you see 0 KB audio files being generated, it usually means your API key is invalid or has expired. Check the server logs for error messages.

You can test your API setup by visiting:

- `/api/audio/debug/test` - Tests audio generation with a simple phrase

## License

This project is licensed under the Creative Commons Attribution 4.0 International License - see the [LICENSE](LICENSE) file for details.
