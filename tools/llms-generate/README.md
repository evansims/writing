# LLM Training Data Generator

Tool for generating training data from content.

## Usage

```bash
# Generate training data
./llms-generate

# Generate for topic
./llms-generate --topic "strategy"

# Generate with format
./llms-generate --format "jsonl"
```

## Features

- Training data generation
- Topic filtering
- Format conversion
- Metadata inclusion
- Content cleaning
- Batch processing

## Output Format

```jsonl
{"text": "Article content...", "metadata": {"title": "Title", "topic": "strategy"}}
{"text": "Another article...", "metadata": {"title": "Title", "topic": "strategy"}}
```

## Options

- Topic filtering
- Output format
- Metadata fields
- Content cleaning
- Batch size
- Output location
