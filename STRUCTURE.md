This repository contains the source code and content for the personal website of Evan Sims (@hello@evansims.com). Although it is intended for personal use in maintaining the site, the repository is open source for educational purposes.

# License

Everything in this repository is licensed under the CC BY-NC-SA 4.0 deed.

# Repository Structure

- `.cursor/` - Cursor configurations for the project.
- `.github/` - GitHub Actions configurations for the project.
- `.vscode/` - VSCode/Cursor/Windsurf configurations for the project.
- `api/` - Python-based backend for the site. Uses Vercel Fluid Compute. See [API](#API) for more information.
- `app/` - Next.js-based frontend for the site. Uses Vercel. See [App](#App) for more information.
- `configuration/` - YAML configuration files for the site. See [Configuration](#Configuration) for more information.
- `content/` - Content for the site. See [Content](#Content) for more information.
- `docs/` - Documentation for the project.
- `tools/` - Rust-based CLI tools for managing the site and its content. See [Tools](#Tools) for more information.

## API

The `/api` directory contains the Python-based backend APIs for the site. These are served using Vercel Fluid Compute.

```
api/
├── common/
│   ├── filesystem.py
│   └── validation.py
├── content.py
├── images.py
├── index.py
├── llms.py
├── rss.py
└── sitemap.py
```

- `common/filesystem.py` - Common filesystem utilities for use in the API.
- `common/validation.py` - Common validation utilities for use in the API.
- `content.py` - Serves the content for the site.
- `images.py` - Serves the images for the site.
- `index.py` - Serves a Hello World API response, for testing service status.
- `llms.py` - Serves the `llms.txt` and `llms-full.txt` files.
- `rss.py` - Serves the RSS feeds for the site.
- `sitemap.py` - Serves the sitemap for the site.

## App

The `/app` directory contains the Next.js-based frontend for the site.

Example:

```
app/
├── assets/
├── [slug]/
│   └── page.tsx
└── not-found.tsx
```

- `assets/` - Static assets for the site.
- `[slug]/page.tsx` - Dynamically serves the content for the site. Fetches the content from the backend API.
- `not-found.tsx` - Serves the 404 page for the site.

## Configuration

The `/configuration` directory contains a series of YAML configuration files.

```
configuration/
├── api.yml
├── images.yml
├── llms.yml
├── rss.yml
├── site.yml
├── sitemap.yml
└── ui.yml
```

### API Configuration

The `api.yml` file contains configuration for the API.

### RSS Configuration

The `rss.yml` file contains configuration for the RSS feeds.

### Images Configuration

The `images.yml` file contains configuration for the images.

### LLMs Configuration

The `llms.yml` file contains configuration for the LLMs.

### Site Configuration

The `site.yml` file contains configuration for the site.

### Sitemap Configuration

The `sitemap.yml` file contains configuration for the sitemap.

### UI Configuration

The `ui.yml` file contains configuration for the UI.

## Content

The `/content` directory contains nested files and folders that organize site content. The structure is flexible and allows for a variety of different content types. It is designed to be easy to understand and navigate, and highly manageable using tools like Obsidian.

Example:

```
content/
├── about/
│   ├── about.md
│   └── about.png
├── mindset/
│   └── curb-your-enthusiasm/
│       └── curb-your-enthusiasm.md
├── podcast/
│   └── episode-1/
│       ├── episode-1.md
│       ├── audio.mp3
│       └── video.mp4
└── example/
    └── another-example/
        └── deeply-nested-page/
            └── deeply-tested-example.md
```

This content structure would result in the following pages:

- https://evansims.com/about/
- https://evansims.com/mindset/curb-your-enthusiasm/
- https://evansims.com/podcast/episode-1/
- https://evansims.com/example/another-example/deeply-tested-page/

When a folder contains a Markdown file with a matching filename, it is treated as the index page for that folder. Any other Markdown files inside a folder are ignored, and are not accessible through the site. These might be used for internal documentation, for example.

However, other file types in the folder are accessible through the folder path. For example, the audio file for episode 1 would be accessible at https://evansims.com/podcast/episode-1/audio.mp3.

### Content Format

Content is stored in Markdown files, with optional frontmatter.

Example:

```markdown
---
title: "Page Title"
description: "A brief description of the content."
created: "2024-02-18"
updated: "2025-03-19"
tags: ["mindset", "philosophy", "engineering"]
banner: "banner.png"
---

# Title

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam venenatis sapien at metus condimentum, vel fermentum nunc tincidunt. Fusce lacinia magna vel justo faucibus, in efficitur erat vestibulum. Donec commodo, orci nec vestibulum tincidunt, eros magna feugiat nulla, ac rutrum eros lectus vel nisi. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Curabitur volutpat diam in magna elementum, vel ultrices nunc dictum. Proin sagittis, magna at convallis ullamcorper, nunc nisi ultrices orci, eget tempus nibh nunc vel dolor. Sed egestas velit at enim commodo, vel pharetra magna varius.
```

Any arbitrary properties can be added to the frontmatter. This is not an exhaustive list. However, the following properties are suggested:

| Key           | Description                                                            | Example                                    |
| ------------- | ---------------------------------------------------------------------- | ------------------------------------------ |
| `title`       | The title of the page.                                                 | `"Page Title"`                             |
| `description` | A brief description of the content.                                    | `"A brief description of the content."`    |
| `created`     | The date of when the page was first published.                         | `"2024-02-18"`                             |
| `updated`     | The date of when the page was last updated.                            | `"2025-03-19"`                             |
| `tags`        | A list of tags associated with the page.                               | `["mindset", "philosophy", "engineering"]` |
| `banner`      | Filename of an image to use as the featured banner image for the page. | `"banner.png"`                             |
