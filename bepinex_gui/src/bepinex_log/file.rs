use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};
use zip::write::FileOptions;

use crate::{file_explorer_utils, settings::get_directory_full_path};

pub fn open_file_explorer_to_log_file_and_zip_if_needed(
    bepinex_log_output_file_full_path: &PathBuf,
) {
    if let Ok(log_file_metadata) = fs::metadata(&bepinex_log_output_file_full_path) {
        let file_size_bytes = log_file_metadata.len();
        const ONE_MEGABYTE: u64 = 1000000;
        // check log file size, if its more than size limit, just zip it
        if file_size_bytes >= ONE_MEGABYTE {
            let zip_file_full_path = bepinex_log_output_file_full_path
                .parent()
                .unwrap()
                .join("zipped_log.zip");
            match zip(&zip_file_full_path, &bepinex_log_output_file_full_path) {
                Ok(_) => {
                    file_explorer_utils::highlight_path_in_explorer(&zip_file_full_path);
                }
                Err(e) => {
                    tracing::error!("Failed zipping: {}", e.to_string());
                }
            }
        } else {
            file_explorer_utils::highlight_path_in_explorer(bepinex_log_output_file_full_path);
        }
    }
}

pub fn zip<P: AsRef<Path>, P2: AsRef<Path>>(
    output_zip_file_path: P,
    input_log_file_path: P2,
) -> zip::result::ZipResult<()> {
    let zip_file = std::fs::File::create(&output_zip_file_path).unwrap();

    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip.start_file("LogOutput.log", options)?;
    let mut log_file_buffer = BufReader::new(File::open(input_log_file_path)?);
    let mut zip_buf_writer = BufWriter::new(zip);
    std::io::copy(&mut log_file_buffer, &mut zip_buf_writer)?;

    // zip.write_all()?;

    // zip.finish()?;
    Ok(())
}

pub(crate) fn file_full_path() -> Option<PathBuf> {
    if let Some(directory_full_path) = get_directory_full_path() {
        if std::fs::create_dir_all(&directory_full_path).is_ok() {
            return Some(directory_full_path.join("log.txt"));
        }
    }

    None
}
