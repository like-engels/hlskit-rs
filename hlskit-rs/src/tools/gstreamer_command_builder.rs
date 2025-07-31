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
    hlskit_error::GStreamerCommandBuilderError,
    internals::hls_output_config::{HlsOutputConfig, HlsOutputEncryptionConfig},
};

#[derive(Debug, Default)]
pub struct GStreamerCommand {
    input_path: PathBuf,
    output_path: PathBuf,
    width: i32,
    height: i32,
    bitrate: i32,
    hls_config: Option<HlsOutputConfig>,
}

#[derive(Debug, Default)]
pub struct GStreamerCommandBuilder {
    command: GStreamerCommand,
    has_input: bool,
    has_output: bool,
    has_dimensions: bool,
    has_bitrate: bool,
    errors: Vec<GStreamerCommandBuilderError>,
}

impl GStreamerCommandBuilder {
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
            self.errors
                .push(GStreamerCommandBuilderError::InvalidDimensions(
                    "Width and height must be positive.".to_string(),
                ));
        }
        self.command.width = width;
        self.command.height = height;
        self.has_dimensions = true;
        self
    }

    pub fn bitrate(mut self, kbps: i32) -> Self {
        if kbps <= 0 {
            self.errors
                .push(GStreamerCommandBuilderError::InvalidBitrate(
                    "Bitrate must be a positive value.".to_string(),
                ));
        }
        self.command.bitrate = kbps;
        self.has_bitrate = true;
        self
    }

    pub fn enable_hls(
        mut self,
        segment_pattern: &str,
        playlist_type: Option<&str>,
        base_url: Option<&str>,
        encryption: Option<HlsOutputEncryptionConfig>,
        hls_time: i32,
    ) -> Self {
        if !segment_pattern.contains('%') {
            self.errors
                .push(GStreamerCommandBuilderError::InvalidConfig(
                    "Segment pattern must contain a printf-style specifier (e.g., %05d)."
                        .to_string(),
                ));
        }

        self.command.hls_config = Some(HlsOutputConfig {
            segment_filename_pattern: segment_pattern.to_string(),
            playlist_type: playlist_type.map(String::from),
            base_url: base_url.map(String::from),
            encryption_config: encryption,
            hls_time,
        });

        self
    }

    pub fn build(&mut self) -> Result<Vec<String>, GStreamerCommandBuilderError> {
        if !self.errors.is_empty() {
            return Err(self.errors.remove(0));
        }

        if !self.has_input {
            return Err(GStreamerCommandBuilderError::MissingInput);
        }

        if !self.has_output && self.command.hls_config.is_none() {
            return Err(GStreamerCommandBuilderError::MissingOutput);
        }

        if !self.has_dimensions {
            return Err(GStreamerCommandBuilderError::InvalidDimensions(
                "Must specify video dimensions.".to_string(),
            ));
        }

        if !self.has_bitrate {
            return Err(GStreamerCommandBuilderError::InvalidBitrate(
                "Must specify video bitrate.".to_string(),
            ));
        }

        Ok(self.command.to_args())
    }
}

impl GStreamerCommand {
    pub fn to_args(&self) -> Vec<String> {
        let mut args = vec!["gst-launch-1.0".to_string()];

        args.push("filesrc".to_string());
        args.push(format!("location={}", self.input_path.display()));
        args.push("! decodebin".to_string());
        args.push("! videoconvert ! videoscale".to_string());
        args.push(format!(
            "! video/x-raw,width={},height={}",
            self.width, self.height
        ));
        args.push(format!(
            "! x264enc bitrate={} speed-preset=medium tune=zerolatency",
            self.bitrate
        ));
        args.push("! mpegtsmux".to_string());

        if let Some(hls) = &self.hls_config {
            args.push("! hlssink".to_string());

            args.push(format!("playlist-location={}", self.output_path.display()));

            args.push(format!("location={}", hls.segment_filename_pattern));
            args.push(format!("target-duration={}", hls.hls_time));

            if let Some(enc) = &hls.encryption_config {
                args.push(format!("key-file={}", enc.encryption_key_path));

                // Use base_url as the prefix to form the full key URI
                let key_filename = std::path::Path::new(&enc.encryption_key_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                if let Some(base_url) = &hls.base_url {
                    let mut key_uri = base_url.clone();
                    if !key_uri.ends_with('/') {
                        key_uri.push('/');
                    }
                    key_uri.push_str(&key_filename);
                    args.push(format!("key-uri={}", key_uri));
                } else {
                    // Fallback to just using the filename
                    args.push(format!("key-uri={}", key_filename));
                }

                if let Some(iv) = &enc.iv {
                    args.push(format!("iv={}", iv));
                }
            }
        } else {
            args.push("! filesink".to_string());
            args.push(format!("location={}", self.output_path.display()));
        }

        args
    }
}
