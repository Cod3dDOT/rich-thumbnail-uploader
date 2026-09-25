/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

pub(crate) const HELP: &str = "\
Rich Thumbnail Uploader - Upload thumbnails for Discord Rich Presence

USAGE:
    rich-thumbnail-uploader [OPTIONS]

OPTIONS:
    -d, --dimensions <DIMS>    Max width/height of the thumbnail (128-512) [default: 256]
    -s, --service <SERVICE>    Image hosting service [default: catbox]
                               [possible values: imgur, catbox]
    -f, --format <FORMAT>      Output image format [default: png]
                               [possible values: png, jpeg, webp (catbox only)]
    -q, --quality <QUALITY>    JPEG quality (1-100) [default: 80]
        --uid <UID>            Imgur client ID or catbox userhash
        --timeout <SECONDS>    Network timeout (1-255) [default: 10]
    -h, --help                 Print help information
    -V, --version              Print version information

EXAMPLES:
    rich-thumbnail-uploader -s catbox -f webp -d 512
    rich-thumbnail-uploader --service imgur --uid YOUR_CLIENT_ID

For more information, visit: https://github.com/cod3ddot/rich-thumbnail-uploader
";
