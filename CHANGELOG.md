# Changelog

All notable changes to HlsKit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.0

### Added

- GStreamer backend support
- New API Style (activate it by enabling the zenpulse-api feature)
- Video validation using Magic Bytes

### Changed

- Code & architectural improvements and security fixes
- Improved error handling

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- Updated dependencies to mitigate security vulnerabilities

## 0.2.0

### Added

- New HLS AES-128 encryption support
- Custom audio codec support
- Custom audio bitrate support

### Changed

- Refactored Ffmpeg codebase with a new command builder to dynamically build Ffmpeg commands based on the provided settings.
- Added encryption settings to Ffmpeg command builder, now supports AES-128 encryption.
- Added support for custom video codec and bitrate settings within the `HlsVideoProcessingSettings` struct.
- Started codebase improvements to decouple Ffmpeg and open to new backends such as GStreamer.

## 0.1.3

### Added

- N/A

### Changed

- Updated dependencies
- Now using temporal files instead of using stdin to input videos to Ffmpeg

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- Fixed segment name collision in multiple resolutions
- Fixed segment name not matching the m3u8 playlist segment naming

### Security

- N/A

## 1.0.2

### Added

- Added LGPL V3 or later licensing
- Added CLA
- Added Changelog file
- Added workflow files for deploying HlsKit to crates.io
- Added PR Template
- Updated CONTRIBUTING.md and README.md files

### Changed

- N/A

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A

## [0.1.0] - 2025-04-09

### Added

- Initial release of HlsKit, a high-performance Rust library for converting MP4 files to HLS-compatible outputs.
- Support for FFmpeg-based video processing with adaptive bitrate output.
- Core modules: `hls_video`, `hls_video_processing_settings`, `hls_video_processing_service`, and utilities in `tools/`.
- Asynchronous execution using `tokio` for non-blocking video processing.
- Custom error handling via `hlskit_error.rs`.
- Unit and integration tests for key functionality.

### Changed

- N/A

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A

---

Thank you for using HlsKit! Contributions and feedback are welcome via the [HlsKit repository](https://github.com/like-engels/hlskit-rs).
