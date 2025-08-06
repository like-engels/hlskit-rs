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

use std::{fs::File, io::Read, path::PathBuf};

use crate::{
    models::hls_video::{HlsVideoResolution, HlsVideoSegment},
    tools::hlskit_error::HlsKitError,
};

pub fn read_playlist_and_segments(
    playlist_filename: &str,
    segment_filename: &str,
    resolution: (i32, i32),
    stream_index: i32,
) -> Result<HlsVideoResolution, HlsKitError> {
    let mut resolution = HlsVideoResolution {
        resolution,
        playlist_name: format!("playlist_{stream_index}.m3u8"),
        playlist_data: Vec::new(),
        segments: Vec::new(),
    };

    // Read the playlist file
    let mut playlist_file = File::open(playlist_filename)?;
    playlist_file.read_to_end(&mut resolution.playlist_data)?;

    // Read all segment files
    let mut segment_index = 0;
    loop {
        let segment_path = segment_filename.replace("%03d", &format!("{segment_index:03}"));
        if !PathBuf::from(&segment_path).exists() {
            break;
        }

        let mut segment_file = File::open(&segment_path)?;
        let mut segment_data = Vec::new();
        segment_file.read_to_end(&mut segment_data)?;

        let segment = HlsVideoSegment {
            segment_name: format!("data_{stream_index}_{segment_index:03}.ts"),
            segment_data,
        };
        resolution.segments.push(segment);
        segment_index += 1;
    }

    Ok(resolution)
}
