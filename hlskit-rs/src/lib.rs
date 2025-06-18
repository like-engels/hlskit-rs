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

use std::fs;

use futures::future::try_join_all;
use models::{
    hls_video::{HlsVideo, HlsVideoResolution},
    hls_video_processing_settings::HlsVideoProcessingSettings,
};
use services::hls_video_processing_service::process_video_profile;
use tempfile::TempDir;
use tools::{hlskit_error::HlsKitError, m3u8_tools::generate_master_playlist};

pub mod models;
pub mod services;
pub mod tools;

pub async fn process_video(
    input_bytes: Vec<u8>,
    output_profiles: Vec<HlsVideoProcessingSettings>,
) -> Result<HlsVideo, HlsKitError> {
    let output_dir = TempDir::new()?.keep();

    println!("processing video at: {}", &output_dir.display());

    let tasks: Vec<_> = output_profiles
        .iter()
        .enumerate()
        .map(|(index, profile)| {
            process_video_profile(
                input_bytes.clone(),
                profile.resolution,
                profile.constant_rate_factor,
                profile.preset.value(),
                &output_dir,
                index.try_into().unwrap(),
            )
        })
        .collect();

    let resolution_results: Vec<HlsVideoResolution> = try_join_all(tasks).await?;

    let master_m3u8_data = generate_master_playlist(
        &output_dir,
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

    fs::remove_dir_all(output_dir)?;

    Ok(hls_video)
}
