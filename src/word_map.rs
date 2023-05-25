use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use std::time::Duration;
use tokio::time;

pub struct WordMap {
    root: String,
    url_world_map: Arc<Mutex<HashMap<String, String>>>,
}

impl WordMap {
    pub fn new(root: String) -> Result<WordMap> {
        if Path::new(&root).is_dir() {
            let msg = format!("Directory not found, need a valid dump for audio files.");
            Err(Error::new(ErrorKind::NotFound, msg))
        } else {
            Ok(WordMap {
                root,
                url_world_map: Arc::new(Mutex::new(HashMap::new())),
            })
        }
    }

    pub async fn insert(self, key: String, value: String) {
        let mut map = self.url_world_map.lock().await;
        map.insert(key, value);
    }

    pub async fn try_get(self, key: String) -> Option<String> {
        let map = self.url_world_map.lock().await;
        return map.get(&key).cloned();
    }
}
