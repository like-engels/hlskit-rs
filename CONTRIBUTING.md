# Contribution Guide for HlsKit

Love HlsKit? Join our community! Whether you’re submitting changes directly, forking the project, or building extensions, you’re a contributor to the HlsKit ecosystem. We’d love for you to share your work with us—let’s build something great together!

> HlsKit is licensed under LGPLv3. By modifying or distributing it (e.g., via forks, wrappers, libraries or extensions), you agree to the [HlsKit Contributor License Agreement (CLA)](CLA.md), which ensures our ecosystem thrives.
> Everyone who modifies HlsKit is part of our community! We encourage you to license your changes under LGPLv3 and make them available to others, fostering collaboration across
> To prevent abuse, we encourage developers using modified versions in networked applications to make their source code available, in the spirit of the AGPL

## Introduction

HlsKit is a high-performance Rust library that enables conversion of MP4 video files into adaptive bitrate HLS-compatible outputs using FFmpeg, with upcoming support for GStreamer. We welcome community contributions and believe in fostering a collaborative, inclusive, and technically excellent environment. This guide outlines how to contribute code, report issues, and propose improvements to HlsKit.

## General Guidelines

1. **Be respectful and constructive**: Engage with the community professionally and courteously.
2. **Prioritize clarity and quality**: Write clean, well-documented, and tested code.
3. **Discuss major changes first**: Before working on a new feature or major change, open an issue to discuss it.

## Dependencies

HlsKit relies on the following core dependencies:

- `futures` (with `futures-executor` and `thread-pool` features)
- `tempdir`
- `thiserror`
- `tokio` (with `process` and `io-util` features)

Avoid introducing new dependencies unless necessary. If a new dependency is needed, explain your rationale in an issue for community feedback.

## Project Structure

HlsKit is organized into clearly separated modules:

```
src/
├── lib.rs                      # Public API surface
├── models/                    # Shared types and data structures
│   ├── hls_video.rs
│   ├── hls_video_processing_settings.rs
├── services/                  # Video processing services
│   └── hls_video_processing_service.rs
├── tools/                     # Utilities for ffmpeg command generation, error handling, and playlist generation
│   ├── ffmpeg_command_builder.rs
│   ├── hlskit_error.rs
│   └── m3u8_tools.rs
```

This modular structure promotes reusability and testability.

## Testing

Unit and integration tests are strongly encouraged. All major functions should have tests validating their behavior. Use `cargo test` to ensure your changes don’t break existing functionality.

## Error Handling

HlsKit uses custom errors defined in `tools/hlskit_error.rs`. Use these consistently when adding new features or modifying existing logic.

## Code Review

All contributions must go through code review. Reviews ensure code quality, performance, and safety. Please respond to feedback promptly and revise your pull request as needed.

## Code Style

Follow [Rust’s official style guidelines](https://doc.rust-lang.org/1.0.0/style/). Run `rustfmt` before submitting a pull request to maintain consistency.

## Asynchronous Execution

HlsKit is built on `tokio` and uses asynchronous execution extensively. Ensure new async functions are clearly marked and awaited correctly. Prefer non-blocking code paths.

## Submitting New Features

Before implementing a new feature, create an issue describing it using this format:

**Example Feature Issue:**

**Description:**
Add support for GStreamer as an alternative backend to FFmpeg, allowing users to select the backend at runtime.

**Changes Required:**

- Add a GStreamer-based video processor module.
- Update `process_video` to delegate based on backend choice.
- Update `HlsVideoProcessingSettings` with a backend field.

**Scope of Changes:**

- New backend module
- Configuration updates

**Acceptance Criteria:**

- Users can choose FFmpeg or GStreamer.
- Output playlists and segments are generated identically.

**Test Plans:**

- Unit tests for backend switching logic.
- Integration tests for both backends.

## Submitting a Pull Request (PR)

**Example PR Description:**

**Description:**
This PR adds support for runtime backend selection between FFmpeg and GStreamer. A new field was added to `HlsVideoProcessingSettings` for backend choice, and logic in `process_video` was updated to handle both paths.

**Ticket:**
Closes #15

**Changes Made:**

- Added `VideoBackend` enum
- Implemented GStreamer processor stub
- Updated settings model and processing logic

**Acceptance Criteria:**

- Feature toggle between backends works
- Unit tests cover both code paths

**Test Plans:**

- Run `cargo test` for both FFmpeg and GStreamer paths
- Manually tested playlist output integrity

## Reporting Issues

When filing an issue, please include:

- **Clear title and description**
- **Reproduction steps**
- **Expected vs actual behavior**
- **Logs and environment details (OS, version)**

This helps maintainers resolve problems efficiently.

---

Thank you for contributing to HlsKit! We’re excited to build an efficient and powerful video streaming toolkit with your help.
