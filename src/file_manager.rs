use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;

pub struct FileManager {
    root: String,
    url_file_map: HashMap<String, String>,
}

impl FileManager {
    pub fn new(root: String) -> Result<FileManager> {
        if Path::new(&root).is_dir() {
            let msg = format!("Directory not found, need a valid dump for audio files.");
            Err(Error::new(ErrorKind::NotFound, msg))
        } else {
            Ok(FileManager {
                root,
                url_file_map: HashMap::new(),
            })
        }
    }
}
