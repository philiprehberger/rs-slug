# Changelog

## 0.2.0 (2026-04-27)

- Add Russian Cyrillic transliteration (full alphabet, both cases)
- Add modern Greek transliteration following ISO 843
- Add `SlugBuilder::lowercase(bool)` to preserve original case
- Add `SlugBuilder::ascii_only(bool)` to drop unmapped non-ASCII characters
- Add top-level `is_valid_slug()` validator

## 0.1.8 (2026-03-31)

- Standardize README to 3-badge format with emoji Support section
- Update CI checkout action to v5 for Node.js 24 compatibility

## 0.1.7 (2026-03-27)

- Add GitHub issue templates, PR template, and dependabot configuration
- Update README badges and add Support section

## 0.1.6 (2026-03-17)

- Add readme, rust-version, documentation to Cargo.toml
- Add Development section to README

## 0.1.5 (2026-03-16)

- Update install snippet to use full version

## 0.1.4 (2026-03-16)

- Add README badges
- Synchronize version across Cargo.toml, README, and CHANGELOG

## 0.1.0 (2026-03-15)

- Initial release
- `slugify()` function for quick slug generation
- `SlugBuilder` for configurable slug generation
- Unicode to ASCII transliteration for Latin-script characters
- Consecutive separator collapsing and trimming
- Word-boundary-aware truncation with max length
