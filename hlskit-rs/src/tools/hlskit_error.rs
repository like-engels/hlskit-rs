use thiserror::Error;

#[derive(Error, Debug)]
pub enum HlsKitError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("[HlsKit] Failed to spawn Ffmpeg: {error:?}")]
    FfmpegError { error: String },
    #[error("File {file_path:?} not found")]
    FileNotFound { file_path: String },
}
