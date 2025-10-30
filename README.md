# rs-slug

[![CI](https://github.com/philiprehberger/rs-slug/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-slug/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-slug.svg)](https://crates.io/crates/philiprehberger-slug)
[![GitHub release](https://img.shields.io/github/v/release/philiprehberger/rs-slug)](https://github.com/philiprehberger/rs-slug/releases)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-slug)](https://github.com/philiprehberger/rs-slug/commits/main)
[![License](https://img.shields.io/github/license/philiprehberger/rs-slug)](LICENSE)
[![Bug Reports](https://img.shields.io/github/issues/philiprehberger/rs-slug/bug)](https://github.com/philiprehberger/rs-slug/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![Feature Requests](https://img.shields.io/github/issues/philiprehberger/rs-slug/enhancement)](https://github.com/philiprehberger/rs-slug/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

Unicode-aware slug generation for URL-safe strings

## Installation

```toml
[dependencies]
philiprehberger-slug = "0.1.7"
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


## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this package useful, consider giving it a star on GitHub — it helps motivate continued maintenance and development.

[![LinkedIn](https://img.shields.io/badge/Philip%20Rehberger-LinkedIn-0A66C2?logo=linkedin)](https://www.linkedin.com/in/philiprehberger)
[![More packages](https://img.shields.io/badge/more-open%20source%20packages-blue)](https://philiprehberger.com/open-source-packages)

## License

[MIT](LICENSE)
