[![Rich Thumbnail Uploader](https://cod3d.dev/img/readme-rich_thumbnail_uploader.gif)](https://github.com/cod3ddot/rich-thumbnail-uploader)

<div align="center">
  <a href="https://www.gnu.org/licenses/agpl-3.0" target="_blank" rel="noopener noreferrer">
    <img src="https://www.gnu.org/graphics/agplv3-155x51.png" alt="License: AGPL v3" height="28" />
  </a>

  <a href="https://github.com/cod3ddot/rich-thumbnail-uploader/releases">
    <img src="https://img.shields.io/github/v/tag/cod3ddot/rich-thumbnail-uploader?label=version&style=for-the-badge&color=blue&logo=git&logoColor=white" alt="Version" />
  </a>

  <a href="https://www.rust-lang.org">
    <img src="https://img.shields.io/badge/Rust-orange.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Language: Rust" />
  </a>
</div>

# Rich Thumbnail Uploader

Thumbnail uploader for [`foo_discord_rich`](https://github.com/RemuSalminen/foo_discord_rich) — a plugin for Discord Rich Presence in foobar2000.

Uploads a thumbnail of album artwork to image hosting services and returns a link.

Inspired by [rust-imgur-upload](https://github.com/s0hv/rust-imgur-upload).

## Supported Services

- **Imgur** — converts to PNG
- **Catbox** — converts to WebP or PNG

## Quick Setup

Clone and build:
```bash
git clone https://github.com/your-username/rich-thumbnail-uploader
cd rich-thumbnail-uploader
cargo build --release
```

## Project Structure

```
├── LICENCES/               # REUSE licenses (See README)
├── resources/              # Windows metadata
├── src/
│   ├── models/             # Models for api responses
│   │   ├── imgur.rs
│   │   └── mod.rs
│   ├── uploaders/          # Upload logic per service
│   │   ├── imgur.rs
│   │   ├── catbox.rs
│   │   └── mod.rs
│   ├── config/             # CLI helpers
│   │   ├── cli.rs
│   │   ├── help.rs
│   │   └── mod.rs
│   ├── errors.rs           # Contains error types
│   ├── image_processor.rs  # Generates thumbnails
│   └── main.rs             # Entry point
├── Cargo.toml              # Crate metadata and dependencies
├── build.rs                # Build logic (windows metadata)
├── CHANGES.md              # Changelog
├── COPYING                 # AGPL-3.0-or-later (See README)
└── README.md               # This file
```

## Usage
1. Save executable on disk
2. File -> Preferences -> Discord Rich Presence Integration -> Advanced
3. Set upload command as the path to the executable, with any options you would like

Example: `C:\Users\user\rich-thumbnail-uploader.exe -s catbox -f webp`


## Options

```bash
Rich Thumbnail Uploader - Upload thumbnails for Discord Rich Presence

USAGE:
    rich-thumbnail-uploader [OPTIONS]

OPTIONS:
    -d, --dimensions <DIMS>    Dimensions to resize image to (128-512) [default: 256]
    -s, --service <SERVICE>    Image hosting service [default: catbox]
                               [possible values: imgur, catbox]
    -f, --format <FORMAT>      Output image format [default: png]
                               [possible values: png, webp]
        --uid <UID>           User ID for service authentication
    -h, --help                Print help information
    -V, --version             Print version information
```
## License

This project strives to be [REUSE](https://reuse.software/) compliant.

Generally:
- Documentation is licensed under CC-BY-NC-SA-4.0
- Code is licensed under AGPL-3.0-or-later
- Config files are under CC0-1.0

```
    rich-thumbnail-uploader: thumbnail uploader for foo_discord_rich
    Copyright (C) 2025  cod3ddot@proton.me

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
