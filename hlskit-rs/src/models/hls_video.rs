/// Represents an HLS video segment
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HlsVideoSegment {
    pub segment_name: String,
    pub segment_data: Vec<u8>,
}

/// Represents a video resolution and its corresponding playlist
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HlsVideoResolution {
    pub resolution: (i32, i32),
    pub playlist_name: String,
    pub playlist_data: Vec<u8>,
    pub segments: Vec<HlsVideoSegment>,
}

/// Represents an HLS video with multiple resolutions
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HlsVideo {
    pub master_m3u8_data: Vec<u8>,
    pub resolutions: Vec<HlsVideoResolution>,
}
