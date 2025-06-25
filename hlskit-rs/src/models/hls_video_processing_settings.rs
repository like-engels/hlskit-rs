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

/// Preset options for FFmpeg video processing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfmpegVideoProcessingPreset {
    VerySlow,
    Slower,
    Slow,
    Medium,
    Fast,
    Faster,
    VeryFast,
    SuperFast,
    UltraFast,
}

impl FfmpegVideoProcessingPreset {
    pub fn value(&self) -> &str {
        match self {
            FfmpegVideoProcessingPreset::VerySlow => "veryslow",
            FfmpegVideoProcessingPreset::Slower => "slower",
            FfmpegVideoProcessingPreset::Slow => "slow",
            FfmpegVideoProcessingPreset::Medium => "medium",
            FfmpegVideoProcessingPreset::Fast => "fast",
            FfmpegVideoProcessingPreset::Faster => "faster",
            FfmpegVideoProcessingPreset::VeryFast => "veryfast",
            FfmpegVideoProcessingPreset::SuperFast => "superfast",
            FfmpegVideoProcessingPreset::UltraFast => "ultrafast",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HlsVideoAudioCodec {
    Aac,
    Mp3,
    Vorbis,
}

impl HlsVideoAudioCodec {
    pub fn value(&self) -> &str {
        match self {
            HlsVideoAudioCodec::Aac => "aac",
            HlsVideoAudioCodec::Mp3 => "mp3",
            HlsVideoAudioCodec::Vorbis => "vorbis",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HlsVideoAudioBitrate {
    Low,
    Medium,
    High,
}

impl HlsVideoAudioBitrate {
    pub fn value(&self) -> &str {
        match self {
            HlsVideoAudioBitrate::Low => "128k",
            HlsVideoAudioBitrate::Medium => "256k",
            HlsVideoAudioBitrate::High => "320k",
        }
    }
}

/// Represents the settings for HLS video processing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HlsVideoProcessingSettings {
    pub resolution: (i32, i32),
    pub constant_rate_factor: i32,
    pub audio_codec: HlsVideoAudioCodec,
    pub audio_bitrate: HlsVideoAudioBitrate,
    pub preset: FfmpegVideoProcessingPreset,
}

impl HlsVideoProcessingSettings {
    pub fn new(
        resolution: (i32, i32),
        constant_rate_factor: i32,
        audio_codec: Option<HlsVideoAudioCodec>,
        audio_bitrate: Option<HlsVideoAudioBitrate>,
        preset: FfmpegVideoProcessingPreset,
    ) -> Self {
        Self {
            resolution,
            constant_rate_factor,
            audio_codec: audio_codec.unwrap_or(HlsVideoAudioCodec::Aac),
            audio_bitrate: audio_bitrate.unwrap_or(HlsVideoAudioBitrate::Medium),
            preset,
        }
    }
}
