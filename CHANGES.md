# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- 32-bit build (`rich-thumbnail-uploader-x86.exe`) for 32-bit Windows
- GIF and BMP album art can be used as input

### Fixed

- JPEG output failing for RGBA / grayscale+alpha / 16-bit cover art, and WebP output failing for 16-bit cover art
- Binary required Windows 10; now runs on Windows 7+ like foobar2000 (built for `*-win7-windows-msvc`)
- Application manifest now declares Windows 7/8/8.1 support
- Invalid `--service`/`--format`, out-of-range `--quality`/`--timeout`, and unknown or misspelled arguments are now errors instead of being silently replaced with defaults or ignored
- `-V` prints the version, as documented (`-v` is no longer accepted)
- Help and README now document `-q/--quality`, `--timeout` and the `jpeg` format
- `--timeout` now limits the whole upload

### Changed

- Bump pinned toolchain to nightly-2026-09-25 (rustc 1.100.0)
- Windows resources are built with `winresource` only; dropped `embed-resource` and `resources/app.rc`
- Imgur response parsed without `miniserde` (dependency removed)
- Update all dependencies, including `attohttpc` 0.31 and `image` 0.25.10

### Security

- Uploads no longer follow HTTP redirects, which re-sent the Imgur client ID or Catbox userhash to the redirect target, even over plain http
- Only a single `https://` link from the service is printed; any other response text is an error
- Bump `bytes` to 1.12.1 (RUSTSEC-2026-0007) and `rand` to 0.10.3 (RUSTSEC-2026-0097)

## [0.3.3] - 2025-10-26

### Added

- Optional git hooks can now be installed using `lefthook install`

### Changed

- Enabled jpeg support
- Prepared for lossy webp in the `image` crate
- Removed dependency on vc++ runtimes by statically linking necessary libs
- Link using llvm
- Pin rust version to nightly
- Minor code quality improvements

## [0.3.2] - 2025-07-23

### Changed

- Switch from reqwest to attohttpc
- Minor code quality improvements

## [0.3.1] - 2025-07-19

### Added

- Windows metadata
- Signed releases

## [0.3.0] - 2025-07-18

### Changed

- Now licensed under AGPL
- Packaged executable size reduced to under 600kb by using native-tls and rewriting options with pico-args

## [0.2.0] - 2025-05-16

### Added

- Support for catbox.moe
- Support for specifying upload format (png/webp)

### Changed

- Updated deps

## [0.1.0] - 2025-04-05

### Added

- Initial release
- Support for imgur.com
