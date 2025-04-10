use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use super::hlskit_error::HlsKitError;

pub async fn generate_master_playlist(
    output_dir: &Path,
    resolutions: Vec<(i32, i32)>,
    playlist_filenames: Vec<&str>,
) -> Result<Vec<u8>, HlsKitError> {
    if !output_dir.exists() {
        return Err(HlsKitError::FileNotFound {
            file_path: output_dir.to_string_lossy().into_owned(),
        });
    }

    let master_playlist_path = output_dir.join("master.m3u8");

    {
        // Scope for the write handle
        let mut master_playlist_handler = File::create(&master_playlist_path)?;

        writeln!(master_playlist_handler, "#EXTM3U")?;

        for (index, (width, height)) in resolutions.iter().enumerate() {
            let raw_path = playlist_filenames[index];
            let bandwidth = (index + 1) * 1_500_000;

            writeln!(
                master_playlist_handler,
                "#EXT-X-STREAM-INF:BANDWIDTH={},RESOLUTION={}x{}",
                bandwidth, width, height
            )?;
            writeln!(master_playlist_handler, "{}", raw_path)?;
            println!("[HlsKit] Master playlist created for {}x{}", width, height);
        }

        master_playlist_handler.flush()?;
    }

    // Reopen the file for reading
    let mut master_playlist_handler = File::open(&master_playlist_path)?;
    let mut master_playlist_buffer = Vec::new();
    master_playlist_handler.read_to_end(&mut master_playlist_buffer)?;

    Ok(master_playlist_buffer)
}
