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

/// Represents the settings for HLS video processing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HlsVideoProcessingSettings {
    pub resolution: (i32, i32),
    pub constant_rate_factor: i32,
    pub preset: FfmpegVideoProcessingPreset,
}
