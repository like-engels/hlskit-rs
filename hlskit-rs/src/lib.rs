// SPDX-License-Identifier: LGPL-3.0-only
/*
 * Copyright © 2025 The HlsKit Project
 *
 * This software is licensed under the GNU Lesser General Public License v3.0 (LGPLv3).
 * All contributions adhere to the LGPLv3 and the HlsKit Contributor License Agreement (CLA).
 * A copy of the LGPLv3 can be found at https://www.gnu.org/licenses/lgpl-3.0.html
 *
 * HlsKit Contributor License Agreement
 *
 * By contributing to or modifying HlsKit, you agree to the following terms:
 *
 * 1. Collective Ownership:
 * The HlsKit project incorporates original code and all contributions as a collective work,
 * licensed under LGPLv3. Once submitted, contributions become part of the shared HlsKit
 * ecosystem and cannot be reclaimed, reassigned, or withdrawn. Contributions to your own
 * forks remain yours unless submitted here, at which point they join this collective whole under LGPLv3.
 *
 * 2. Definition of Contribution:
 * You are considered a contributor if you modify the library in any form (including forks,
 * wrappers, libraries, or extensions that alter its behavior), whether or not you submit
 * your changes directly to this repository. All such modifications are part of the broader
 * HlsKit ecosystem and are subject to this CLA.
 *
 * 3. Distribution of Modifications:
 * If you distribute a modified version of HlsKit, you must license your modifications under
 * LGPLv3 (with source code available as required by the license) and ensure they are
 * adoptable by the HlsKit ecosystem (publicly available and compatible).
 *
 * 4. Networked Use of Modifications:
 * If you use a modified version of HlsKit in a networked application, you must provide the
 * source code of your modifications under LGPLv3 and notify the HlsKit project
 * (e.g., via email to [higashikataengels@icloud.com]). This does not apply to the use of
 * the unmodified library in proprietary software, which remains permissible under LGPLv3.
 *
 * 5. Scope:
 * These terms apply to all contributions and modifications derived from the HlsKit project.
 * The use of the unmodified library in proprietary software is governed solely by the LGPLv3.
 */

use std::io::{Read, Write};
use std::{ffi::OsStr, fs, path::PathBuf};

use futures::future::try_join_all;
use models::{
    hls_video::{HlsVideo, HlsVideoResolution},
    hls_video_processing_settings::HlsVideoProcessingSettings,
};

use tempfile::TempDir;
use tools::{hlskit_error::HlsKitError, m3u8_tools::generate_master_playlist};

use crate::backends::ffmpeg_backend::FfmpegBackend;
use crate::traits::video_processing_backend::VideoProcessingBackend;
use crate::{
    tools::hlskit_error::VideoValidatableErrors,
    traits::video_validatable::{VideoInputPathGuard, VideoValidatable},
};

pub mod backends;
pub mod models;
pub mod tools;
pub mod traits;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoInputType {
    InMemoryFile(Vec<u8>),
    FilePath(String),
}

impl VideoValidatable for VideoInputType {
    fn validate(&self) -> Result<VideoInputPathGuard, VideoValidatableErrors> {
        fn is_valid_magic_bytes(buf: &[u8], ext: &str) -> bool {
            match ext {
                "mp4" | "mov" => buf.len() >= 8 && &buf[4..8] == b"ftyp",
                "mkv" => buf.len() >= 4 && &buf[0..4] == b"\x1A\x45\xDF\xA3",
                "avi" => buf.len() >= 12 && &buf[0..4] == b"RIFF" && &buf[8..12] == b"AVI ",
                _ => false,
            }
        }

        let valid_video_extensions = ["mp4", "mkv", "avi", "mov"];

        match &self {
            VideoInputType::InMemoryFile(video_data) => {
                if video_data.is_empty() {
                    return Err(VideoValidatableErrors::EmptyVideoInput);
                }

                let mut valid = false;
                for ext in &valid_video_extensions {
                    if is_valid_magic_bytes(video_data, ext) {
                        valid = true;
                        break;
                    }
                }

                if !valid {
                    return Err(VideoValidatableErrors::InvalidFormat);
                }

                let mut temp_file = tempfile::NamedTempFile::new().map_err(|_| {
                    VideoValidatableErrors::InvalidVideoInput {
                        error: "Failed to create temp file".to_string(),
                    }
                })?;

                temp_file.write_all(video_data).map_err(|_| {
                    VideoValidatableErrors::InvalidVideoInput {
                        error: "Failed to write to temp file".to_string(),
                    }
                })?;

                let path = temp_file.path().to_str().unwrap().to_string();
                Ok(VideoInputPathGuard {
                    path,
                    temp_file: Some(temp_file),
                })
            }
            VideoInputType::FilePath(path) => {
                if path.is_empty() {
                    return Err(VideoValidatableErrors::EmptyVideoInput);
                }

                let pathbuf = PathBuf::from(path);

                if !pathbuf.exists() {
                    return Err(VideoValidatableErrors::FileNotFound);
                }

                if !pathbuf.is_file() {
                    return Err(VideoValidatableErrors::InvalidVideoInput {
                        error: "The given video is not a file".to_string(),
                    });
                }

                let ext = pathbuf
                    .extension()
                    .unwrap_or(OsStr::new("invalid"))
                    .to_str()
                    .unwrap_or("invalid")
                    .to_lowercase();

                if !valid_video_extensions.contains(&ext.as_str()) {
                    return Err(VideoValidatableErrors::InvalidVideoInput {
                        error: "The given video hasn't a valid extension".to_string(),
                    });
                }

                let mut file = match std::fs::File::open(&pathbuf) {
                    Ok(f) => f,
                    Err(_) => {
                        return Err(VideoValidatableErrors::FileNotFound);
                    }
                };

                let mut buf = [0u8; 16];

                let n = match file.read(&mut buf) {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(VideoValidatableErrors::InvalidVideoInput {
                            error: "Failed to read video file for validation".to_string(),
                        });
                    }
                };

                if !is_valid_magic_bytes(&buf[..n], &ext) {
                    return Err(VideoValidatableErrors::InvalidFormat);
                }

                Ok(VideoInputPathGuard {
                    path: path.clone(),
                    temp_file: None,
                })
            }
        }
    }
}

impl Default for VideoInputType {
    fn default() -> Self {
        VideoInputType::InMemoryFile(vec![])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoProcessorEncryptionSettings {
    pub encryption_key_url: String,
    pub encryption_key_path: String,
    pub iv: Option<String>,
}

pub async fn process_video(
    input_bytes: Vec<u8>,
    output_profiles: Vec<HlsVideoProcessingSettings>,
) -> Result<HlsVideo, HlsKitError> {
    let backend = FfmpegBackend;
    process_video_internal::<FfmpegBackend>(
        VideoInputType::InMemoryFile(input_bytes),
        output_profiles,
        None,
        backend,
    )
    .await
}

pub async fn process_video_from_path(
    video_path: &str,
    output_profiles: Vec<HlsVideoProcessingSettings>,
) -> Result<HlsVideo, HlsKitError> {
    let backend = FfmpegBackend;
    process_video_internal::<FfmpegBackend>(
        VideoInputType::FilePath(video_path.to_string()),
        output_profiles,
        None,
        backend,
    )
    .await
}

pub async fn process_video_with_encrypted_segments(
    input_bytes: Vec<u8>,
    output_profiles: Vec<HlsVideoProcessingSettings>,
    encryption_key_url: String,
    encryption_key_path: String,
    iv: Option<String>,
) -> Result<HlsVideo, HlsKitError> {
    let backend = FfmpegBackend;
    let encryption = Some(VideoProcessorEncryptionSettings {
        encryption_key_url,
        encryption_key_path,
        iv,
    });
    process_video_internal::<FfmpegBackend>(
        VideoInputType::InMemoryFile(input_bytes),
        output_profiles,
        encryption,
        backend,
    )
    .await
}

// Internal helper function to avoid code duplication
async fn process_video_internal<V: VideoProcessingBackend>(
    input: VideoInputType,
    output_profiles: Vec<HlsVideoProcessingSettings>,
    encryption: Option<VideoProcessorEncryptionSettings>,
    backend: V,
) -> Result<HlsVideo, HlsKitError> {
    let input_dir_guard = &input.validate()?;

    let temp_file_guard = input_dir_guard.temp_file.as_ref();

    let input_path = match temp_file_guard {
        Some(temp_file) => temp_file.path().to_string_lossy().to_string(),
        None => input_dir_guard.path.clone(),
    };

    let output_dir = TempDir::new()?;
    let output_dir_path = output_dir.path();

    let tasks: Vec<_> = output_profiles
        .iter()
        .enumerate()
        .map(|(index, profile)| {
            backend.process_profile(
                input_path.clone(),
                profile,
                output_dir_path,
                index as i32,
                encryption.as_ref(),
            )
        })
        .collect();

    let resolution_results: Vec<HlsVideoResolution> = try_join_all(tasks).await?;

    let master_m3u8_data = generate_master_playlist(
        output_dir_path,
        resolution_results
            .iter()
            .map(|result| result.resolution)
            .collect(),
        resolution_results
            .iter()
            .map(|result| result.playlist_name.as_str())
            .collect(),
    )
    .await?;

    let hls_video = HlsVideo {
        master_m3u8_data,
        resolutions: resolution_results,
    };

    fs::remove_dir_all(output_dir_path)?;
    Ok(hls_video)
}

#[cfg(feature = "zenpulse-api")]
pub mod prelude {
    use std::fs;

    use futures::future::try_join_all;
    use tempfile::TempDir;

    use crate::{
        models::{
            hls_video::{HlsVideo, HlsVideoResolution},
            hls_video_processing_settings::HlsVideoProcessingSettings,
        },
        tools::{hlskit_error::HlsKitError, m3u8_tools::generate_master_playlist},
        traits::{
            video_processing_backend::VideoProcessingBackend, video_validatable::VideoValidatable,
        },
        VideoProcessorEncryptionSettings,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct VideoProcessor<B, S>
    where
        B: VideoProcessingBackend + Default,
        S: VideoValidatable + Default,
    {
        input_video_path: S,
        output_profiles: Vec<HlsVideoProcessingSettings>,
        encryption_string: Option<VideoProcessorEncryptionSettings>,
        backend: B,
    }

    impl<B, S> Default for VideoProcessor<B, S>
    where
        B: VideoProcessingBackend + Default,
        S: VideoValidatable + Default,
    {
        fn default() -> Self {
            Self {
                input_video_path: Default::default(),
                output_profiles: Default::default(),
                encryption_string: Default::default(),
                backend: Default::default(),
            }
        }
    }

    impl<B: VideoProcessingBackend + Default, S: VideoValidatable + Default> VideoProcessor<B, S> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_video_input(mut self, video: S) -> Self {
            self.input_video_path = video;
            self
        }

        pub fn with_output_profiles(mut self, profiles: Vec<HlsVideoProcessingSettings>) -> Self {
            self.output_profiles = profiles;
            self
        }

        pub fn with_encryption(mut self, encryption: VideoProcessorEncryptionSettings) -> Self {
            self.encryption_string = Some(encryption);
            self
        }

        pub fn with_backend(mut self, backend: B) -> Self {
            self.backend = backend;
            self
        }

        pub async fn process_video(&self) -> Result<HlsVideo, HlsKitError> {
            let input_guard = self.input_video_path.validate()?;

            let temp_file_guard = input_guard.temp_file.as_ref();

            let input_path = match temp_file_guard {
                Some(temp_file) => temp_file.path().to_string_lossy().to_string(),
                None => input_guard.path.clone(),
            };

            let output_dir = TempDir::new()?;
            let output_dir_path = output_dir.path();

            let tasks: Vec<_> = self
                .output_profiles
                .iter()
                .enumerate()
                .map(|(index, profile)| {
                    self.backend.process_profile(
                        input_path.clone(),
                        profile,
                        output_dir_path,
                        index as i32,
                        self.encryption_string.as_ref(),
                    )
                })
                .collect();

            let resolution_results: Vec<HlsVideoResolution> = try_join_all(tasks).await?;

            let master_m3u8_data = generate_master_playlist(
                output_dir_path,
                resolution_results
                    .iter()
                    .map(|result| result.resolution)
                    .collect(),
                resolution_results
                    .iter()
                    .map(|result| result.playlist_name.as_str())
                    .collect(),
            )
            .await?;

            let hls_video = HlsVideo {
                master_m3u8_data,
                resolutions: resolution_results,
            };

            fs::remove_dir_all(output_dir_path)?;
            Ok(hls_video)
        }
    }
}
