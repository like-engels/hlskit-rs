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

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum VideoValidatableErrors {
    #[error("Invalid video format")]
    InvalidFormat,
    #[error("Empty video input")]
    EmptyVideoInput,
    #[error("Invalid video input: {error:?}")]
    InvalidVideoInput { error: String },
    #[error("File not found")]
    FileNotFound,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum VideoProcessingErrors {
    #[error("Missing output file path")]
    MissingOutputPath,
    #[error("Unsupported video bitrate")]
    UnsupportedBitrate,
}

#[derive(Debug, Error)]
pub enum FfmpegCommandBuilderError {
    #[error("Configuration Validation Error: {0}")]
    ConfigurationError(String),
    #[error("Command Build Error: {0}")]
    BuildError(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Conversion Error: {0}")]
    ConversionError(String),
    #[error("Unexpected Internal State: {0}")]
    InternalStateError(String),
    #[error("FFmpeg specific setting error: {0}")]
    FfmpegSettingError(String),
}

#[derive(Debug, Error)]
pub enum GStreamerCommandBuilderError {
    #[error("Invalid dimensions: {0}")]
    InvalidDimensions(String),
    #[error("Invalid bitrate: {0}")]
    InvalidBitrate(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Missing input")]
    MissingInput,
    #[error("Missing output")]
    MissingOutput,
}

#[derive(Error, Debug)]
pub enum HlsKitError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    FFMPEGBUILDER(#[from] FfmpegCommandBuilderError),
    #[error(transparent)]
    GSTREAMERBUILDER(#[from] GStreamerCommandBuilderError),
    #[error(transparent)]
    VideoProcessingError(#[from] VideoProcessingErrors),
    #[error(transparent)]
    VideoValidationError(#[from] VideoValidatableErrors),
    #[error("[HlsKit] Failed to spawn Ffmpeg: {error:?}")]
    FfmpegError { error: String },
    #[error("[HlsKit] Failed to spawn GStreamer: {error:?}")]
    GstreamerError { error: String },
    #[error("File {file_path:?} not found")]
    FileNotFound { file_path: String },
}
