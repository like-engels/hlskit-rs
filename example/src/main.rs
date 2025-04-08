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
