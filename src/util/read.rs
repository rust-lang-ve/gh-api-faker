use crate::error::Error;
use std::fs;

pub fn read_from_static(path: &str) -> Result<String, Error> {
    let path = format!("static/{}", path);

    info!("Reading from: {}", path);
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(err) => Err(Error::StaticFileNotFound(err.to_string())),
    }
}
