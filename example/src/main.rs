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
use std::{env, fs::File, io::Read};

use hlskit::{
    models::hls_video_processing_settings::{
        FfmpegVideoProcessingPreset, HlsVideoProcessingSettings,
    },
    process_video,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting video processing");

    let path = env::current_dir()?;
    println!("Current directory: {}", path.display());

    let mut buf = Vec::new();
    File::open("src/sample.mp4")
        .unwrap()
        .read_to_end(&mut buf)?;

    let result = process_video(
        buf,
        vec![
            HlsVideoProcessingSettings {
                resolution: (1920, 1080),
                constant_rate_factor: 28,
                preset: FfmpegVideoProcessingPreset::Fast,
            },
            HlsVideoProcessingSettings {
                resolution: (1280, 720),
                constant_rate_factor: 28,
                preset: FfmpegVideoProcessingPreset::Fast,
            },
            HlsVideoProcessingSettings {
                resolution: (854, 480),
                constant_rate_factor: 28,
                preset: FfmpegVideoProcessingPreset::Fast,
            },
        ],
    )
    .await?;

    println!("Video processing completed successfully");

    println!(
        "Video master m3u8 file data: {:#?}",
        result.master_m3u8_data
    );

    Ok(())
}
