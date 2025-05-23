#[allow(clippy::too_many_arguments)]
pub async fn build_simple_hls(
    width: i32,
    height: i32,
    crf: i32,
    preset: &str,
    segment_filename: &str,
    playlist_filename: &str,
    hls_time: Option<i32>,
    input_path: &str,
) -> Vec<String> {
    vec![
        "ffmpeg".to_string(),
        "-i".to_string(),
        input_path.to_string(),
        "-vf".to_string(),
        format!("scale={}x{}", width, height),
        "-c:v".to_string(),
        "libx264".to_string(),
        "-crf".to_string(),
        crf.to_string(),
        "-preset".to_string(),
        preset.to_string(),
        "-hls_time".to_string(),
        hls_time.unwrap_or(10).to_string(),
        "-hls_playlist_type".to_string(),
        "vod".to_string(),
        "-hls_segment_filename".to_string(),
        segment_filename.to_string(),
        playlist_filename.to_string(),
    ]
}
