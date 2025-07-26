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

use std::path::{Path, PathBuf};

use crate::tools::{
    hlskit_error::FfmpegCommandBuilderError,
    internals::hls_output_config::{HlsOutputConfig, HlsOutputEncryptionConfig},
};

#[derive(Debug, Default)]
pub struct FfmpegCommand {
    input_path: PathBuf,
    output_path: PathBuf,
    width: i32,
    height: i32,
    crf: i32,
    preset: String,
    hls_config: Option<HlsOutputConfig>,
}

impl FfmpegCommand {
    pub fn to_args(&self) -> Vec<String> {
        let mut args = vec!["ffmpeg".to_string()];

        args.push("-i".to_string());
        args.push(self.input_path.to_str().unwrap_or_default().to_string());

        args.push("-vf".to_string());
        args.push(format!("scale={}x{}", self.width, self.height));

        args.push("-c:v".to_string());
        args.push("libx264".to_string());
        args.push("-crf".to_string());
        args.push(self.crf.to_string());
        args.push("-preset".to_string());
        args.push(self.preset.to_string());

        if let Some(hls_conf) = &self.hls_config {
            args.push("-hls_time".to_string());
            args.push(hls_conf.hls_time.to_string());
            args.push("-hls_playlist_type".to_string());
            args.push(
                hls_conf
                    .playlist_type
                    .as_ref()
                    .cloned()
                    .unwrap_or("vod".to_string()),
            );
            args.push("-hls_segment_filename".to_string());
            args.push(hls_conf.segment_filename_pattern.to_string());

            if let Some(base_url) = &hls_conf.base_url {
                args.push("-hls_base_url".to_string());
                args.push(base_url.to_string());
            }

            if let Some(encryption_config) = &hls_conf.encryption_config {
                args.push("-hls_key_info_file".to_string());
                args.push(encryption_config.encryption_key_path.to_string());
                if let Some(iv) = &encryption_config.iv {
                    args.push("-hls_iv".to_string());
                    args.push(iv.to_string());
                }
            }
        }

        args.push(self.output_path.to_str().unwrap_or_default().to_string());

        args
    }
}

#[derive(Debug, Default)]
pub struct FfmpegCommandBuilder {
    command: FfmpegCommand,
    build_errors: Vec<FfmpegCommandBuilderError>,
    has_input: bool,
    has_output: bool,
    has_dimensions: bool,
    has_crf: bool,
    has_preset: bool,
}

impl FfmpegCommandBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.command.input_path = path.as_ref().to_path_buf();
        self.has_input = true;
        self
    }

    pub fn output<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.command.output_path = path.as_ref().to_path_buf();
        self.has_output = true;
        self
    }

    pub fn dimensions(mut self, width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            self.build_errors
                .push(FfmpegCommandBuilderError::FfmpegSettingError(
                    "Width and height must be positive values.".to_string(),
                ));
        }
        self.command.width = width;
        self.command.height = height;
        self.has_dimensions = true;
        self
    }

    pub fn crf(mut self, value: i32) -> Self {
        if !(0..=51).contains(&value) {
            self.build_errors
                .push(FfmpegCommandBuilderError::FfmpegSettingError(format!(
                    "CRF value {value} is outside the standard range [0-51]."
                )));
        }
        self.command.crf = value;
        self.has_crf = true;
        self
    }

    pub fn preset(mut self, name: &str) -> Self {
        let valid_presets = [
            "ultrafast",
            "superfast",
            "fast",
            "medium",
            "slow",
            "slower",
            "veryslow",
            "none",
        ];
        if !valid_presets.contains(&name) {
            self.build_errors
                .push(FfmpegCommandBuilderError::FfmpegSettingError(format!(
                    "Preset '{name}' is not a recognized FFmpeg preset.",
                )));
        }
        self.command.preset = name.to_string();
        self.has_preset = true;
        self
    }

    pub fn enable_hls(
        mut self,
        segment_filename_pattern: &str,
        playlist_type: Option<&str>,
        base_url: Option<&str>,
        encryption_settings: Option<HlsOutputEncryptionConfig>,
        hls_segment_duration_seconds: i32,
    ) -> Self {
        if segment_filename_pattern.is_empty() || !segment_filename_pattern.contains('%') {
            self.build_errors.push(FfmpegCommandBuilderError::FfmpegSettingError(
                "HLS segment filename pattern must not be empty and should contain a format specifier (e.g., %03d).".to_string(),
            ));
        }
        if hls_segment_duration_seconds <= 0 {
            self.build_errors
                .push(FfmpegCommandBuilderError::FfmpegSettingError(
                    "HLS segment duration must be positive.".to_string(),
                ));
        }

        self.command.hls_config = Some(HlsOutputConfig {
            segment_filename_pattern: segment_filename_pattern.to_string(),
            hls_time: hls_segment_duration_seconds,
            playlist_type: playlist_type.map(|ptype| ptype.to_string()),
            base_url: base_url.map(|url| url.to_string()),
            encryption_config: encryption_settings,
        });
        self
    }

    pub fn build(&mut self) -> Result<Vec<String>, FfmpegCommandBuilderError> {
        if !self.build_errors.is_empty() {
            let error_messages: Vec<String> =
                self.build_errors.iter().map(|e| e.to_string()).collect();
            return Err(FfmpegCommandBuilderError::BuildError(format!(
                "Command configuration failed: [{}]",
                error_messages.join("; ")
            )));
        }

        if !self.has_input || self.command.input_path.as_os_str().is_empty() {
            return Err(FfmpegCommandBuilderError::ConfigurationError(
                "Input path must be set using `.input()`.".to_string(),
            ));
        }
        if !self.has_output || self.command.output_path.as_os_str().is_empty() {
            return Err(FfmpegCommandBuilderError::ConfigurationError(
                "Output path must be set using `.output()`.".to_string(),
            ));
        }
        if !self.has_dimensions {
            return Err(FfmpegCommandBuilderError::ConfigurationError(
                "Output dimensions (width and height) must be set using `.dimensions()`."
                    .to_string(),
            ));
        }
        if !self.has_crf {
            return Err(FfmpegCommandBuilderError::ConfigurationError(
                "CRF (quality) must be set using `.crf()`.".to_string(),
            ));
        }
        if !self.has_preset {
            return Err(FfmpegCommandBuilderError::ConfigurationError(
                "Preset must be set using `.preset()`.".to_string(),
            ));
        }

        if self.command.hls_config.is_some() && self.command.output_path.extension().is_some() {
            self.build_errors.push(FfmpegCommandBuilderError::FfmpegSettingError(
                "When enabling HLS, the output path should typically be a directory, not a specific file extension.".to_string(),
            ));
        }

        Ok(self.command.to_args())
    }
}
