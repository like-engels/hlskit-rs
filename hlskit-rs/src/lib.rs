use std::fs;

use futures::future::try_join_all;
use models::{
    hls_video::{HlsVideo, HlsVideoResolution},
    hls_video_processing_settings::HlsVideoProcessingSettings,
};
use services::hls_video_processing_service::process_video_profile;
use tempdir::TempDir;
use tools::{hlskit_error::HlsKitError, m3u8_tools::generate_master_playlist};

pub mod models;
pub mod services;
pub mod tools;

pub async fn process_video(
    input_bytes: Vec<u8>,
    output_profiles: Vec<HlsVideoProcessingSettings>,
) -> Result<HlsVideo, HlsKitError> {
    let output_dir = TempDir::new("hlskit")?.into_path();

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
