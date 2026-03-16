# rs-slug

[![CI](https://github.com/philiprehberger/rs-slug/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-slug/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-slug.svg)](https://crates.io/crates/philiprehberger-slug)
[![License](https://img.shields.io/github/license/philiprehberger/rs-slug)](LICENSE)

Unicode-aware slug generation for URL-safe strings.

## Installation

```toml
[dependencies]
philiprehberger-slug = "0.1"
```

## Usage

```rust
use philiprehberger_slug::{slugify, SlugBuilder};

// Quick slug generation
let slug = slugify("Hello, World!");
assert_eq!(slug, "hello-world");

// Unicode transliteration
let slug = slugify("Café résumé");
assert_eq!(slug, "cafe-resume");

// Custom configuration
let slug = SlugBuilder::new()
    .separator('_')
    .max_length(20)
    .slugify("A Very Long Title That Should Be Truncated");
```

## API

| Function / Type | Description |
|-----------------|-------------|
| `slugify(input)` | Convert a string to a URL-safe slug |
| `SlugBuilder::new()` | Create a configurable slug builder |
| `.separator(char)` | Set the separator character (default: `-`) |
| `.max_length(usize)` | Set maximum slug length with word-boundary truncation |
| `.replacement(char, &str)` | Add a custom character replacement |
| `.slugify(&self, input)` | Generate a slug with the configured settings |

## License

MIT
