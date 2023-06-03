use std::fs::{OpenOptions, File};

const SWAPFC_TEMP_FILENAME: &str = ".swapfctemp";

pub fn create_temp_file(source: &str) -> Result<(), std::io::Error> {
    todo!("TODO: implement create_temp_file")
}

pub fn swap_file_content(source: &str, destination: &str) -> Result<(), std::io::Error> {
    todo!("TODO: implement swap_file_content")
}

pub fn delete_temp_file() -> Result<(), std::io::Error> {
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
