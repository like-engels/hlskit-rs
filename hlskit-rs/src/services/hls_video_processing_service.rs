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

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tempfile::NamedTempFile;
use tokio::process::Command;

use crate::models::hls_video::HlsVideoSegment;
use crate::{
    models::hls_video::HlsVideoResolution,
    tools::{ffmpeg_command_builder::build_simple_hls, hlskit_error::HlsKitError},
};

pub async fn process_video_profile(
    input_bytes: Vec<u8>,
    resolution: (i32, i32),
    crf: i32,
    preset: &str,
    output_dir: &Path,
    stream_index: i32,
) -> Result<HlsVideoResolution, HlsKitError> {
    let (width, height) = resolution;
    let segment_filename = format!(
        "{}/data_{}_%03d.ts",
        output_dir.to_str().unwrap(),
        stream_index
    );
    let playlist_filename = format!(
        "{}/playlist_{}.m3u8",
        output_dir.to_str().unwrap(),
        stream_index
    );

    let mut temp_file = NamedTempFile::new()?;

    temp_file.write_all(&input_bytes)?;

    temp_file.flush()?;

    temp_file.as_file().sync_all()?;

    let input_path = temp_file.path().to_str().unwrap();

    let command = build_simple_hls(
        width,
        height,
        crf,
        preset,
        &segment_filename,
        &playlist_filename,
        None,
        input_path,
    )
    .await;

    let process = Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| HlsKitError::FfmpegError {
            error: error.to_string(),
        })?;

    let output = process
        .wait_with_output()
        .await
        .map_err(|error| HlsKitError::FfmpegError {
            error: format!("Failed to write to ffmpeg output: {}", error),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(HlsKitError::FfmpegError {
            error: format!(
                "FFmpeg error for resolution {}x{}: {}",
                width, height, stderr
            ),
        });
    }

    let mut hls_resolution = HlsVideoResolution {
        resolution,
        playlist_name: format!("playlist_{}.m3u8", stream_index),
        playlist_data: vec![],
        segments: vec![],
    };

    let mut playlist_handler = File::open(&playlist_filename)?;
    playlist_handler.read_to_end(&mut hls_resolution.playlist_data)?;

    let mut segment_index = 0;

    loop {
        let segment_path = segment_filename.replace("%03d", &format!("{:03}", segment_index));

        if !PathBuf::from(&segment_path).exists() {
            break;
        }

        let mut segment_file_handler = File::open(segment_path)?;
        let mut segment_file_read_buffer: Vec<u8> = Vec::new();
        segment_file_handler.read_to_end(&mut segment_file_read_buffer)?;

        let segment_name = format!("data_{}_%03d.ts", stream_index)
            .replace("%03d", &format!("{:03}", segment_index));

        let segment = HlsVideoSegment {
            segment_name,
            segment_data: segment_file_read_buffer,
        };

        hls_resolution.segments.push(segment);

        segment_index += 1;
    }

    Ok(hls_resolution)
}
