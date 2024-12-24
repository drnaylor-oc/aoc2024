use crate::util::Errors::FileLoadFailed;
use std::fs;
use std::path::Path;

pub fn load_from(filename: &str) -> Result<String, Errors> {
    let path = format!("data{}{}", std::path::MAIN_SEPARATOR, filename);
    let data_file = Path::new(path.as_str());
    fs::read_to_string(data_file).map_err(|x| FileLoadFailed(x))
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Errors {
    FileLoadFailed(std::io::Error),
    NoImplementationError
}

