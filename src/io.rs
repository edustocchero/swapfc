use std::fs::{OpenOptions, File};
use std::io::{Read, Write};

type IOResult = Result<(), std::io::Error>;

const DEFAULT_BUF_SIZE: usize = 1024;
const SWAPFC_TEMP_FILENAME: &str = ".swapfctemp";

pub fn swap_file_content(source: &str, destination: &str) -> IOResult {
    create_temp_file(&source)?;

    let mut source_file = open_file_with_read_permission(&destination)?;
    let mut destination_file = open_file_with_write_permission_truncate(&source)?;
    copy_file_content(&mut source_file, &mut destination_file)?;

    let mut source_file = open_file_with_read_permission(SWAPFC_TEMP_FILENAME)?;
    let mut destination_file = open_file_with_write_permission_truncate(&destination)?;
    copy_file_content(&mut source_file, &mut destination_file)?;

    delete_temp_file()?;

    Ok(())
}

fn copy_file_content(source: &mut File, destination: &mut File) -> IOResult {
    let mut file_has_content = true;

    while file_has_content {
        let mut buf = [0u8; DEFAULT_BUF_SIZE];
        let result_read = source.read(&mut buf)?;

        match result_read {
            0 => {
                file_has_content = false;
            }
            size => {
                destination.write(&mut buf[..size])?;
            }
        }
    }

    destination.flush()?;
    Ok(())
}

fn create_temp_file(source_path: &str) -> IOResult {
    let mut source_file = open_file_with_read_permission(source_path)?;
    let mut temp_file = File::create(SWAPFC_TEMP_FILENAME)?;

    copy_file_content(&mut source_file, &mut temp_file)
}

fn delete_temp_file() -> IOResult {
    std::fs::remove_file(SWAPFC_TEMP_FILENAME)
}

fn open_file_with_read_permission(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .read(true)
        .open(path)
}

fn open_file_with_write_permission_truncate(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
}
