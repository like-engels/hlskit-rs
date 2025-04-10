use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Stdio;
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

    let command = build_simple_hls(
        width,
        height,
        crf,
        preset,
        &segment_filename,
        &playlist_filename,
        None,
    )
    .await;

    let mut process = Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| HlsKitError::FfmpegError {
            error: error.to_string(),
        })?;

    if let Some(mut stdin) = process.stdin.take() {
        tokio::io::AsyncWriteExt::write_all(&mut stdin, &input_bytes)
            .await
            .map_err(|error| HlsKitError::FfmpegError {
                error: format!("Failed to write to ffmpeg stdin: {}", error),
            })?;
    }

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

        let segment = HlsVideoSegment {
            segment_name: format!("segment_{}", segment_index),
            segment_data: segment_file_read_buffer,
        };

        hls_resolution.segments.push(segment);

        segment_index += 1;
    }

    Ok(hls_resolution)
}
