use config::{Config, ConfigError, FileFormat};
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::create_dir;
use std::fs::File;
use std::io::{copy, Cursor};
use std::path::{self, Path};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Clone)]
struct TTSConfig {
    api_key: String,
    voice: String,
    speed: u32,
    download_location: Box<Path>,
    url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TTSResponse {
    error: String,
    r#async: String,
    request_id: String,
    message: String,
}

#[derive(Clone, Debug)]
pub struct FPTClient {
    config: TTSConfig,
    headers: HeaderMap,
    client: Client,
    url_map: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Debug)]
pub enum DownloadError {
    ReqwestError(reqwest::Error),
    NoWordFoundForURL(),
    FileCreationError(std::io::Error),
}

impl From<reqwest::Error> for DownloadError {
    fn from(error: reqwest::Error) -> Self {
        DownloadError::ReqwestError(error)
    }
}

impl From<std::io::Error> for DownloadError {
    fn from(error: std::io::Error) -> Self {
        DownloadError::FileCreationError(error)
    }
}

impl FPTClient {
    pub fn new(url_map: Arc<Mutex<HashMap<String, String>>>) -> Result<FPTClient, ConfigError> {
        let tts_config: TTSConfig = Config::builder()
            .add_source(config::File::new("config/FPTClient.toml", FileFormat::Toml))
            .build()?
            .get("ttsConfig")?;

        let mut headers = HeaderMap::new();
        headers.insert("api-key", tts_config.api_key.parse().unwrap());
        headers.insert("voice", tts_config.voice.parse().unwrap());
        headers.insert("speed", tts_config.speed.into());

        create_dir(tts_config.download_location.clone())
            .expect("Directory could not be created, check config.");

        return Ok(FPTClient {
            config: tts_config,
            headers,
            client: reqwest::Client::new(),
            url_map,
        });
    }

    pub async fn request_tts(self, phrase: String) -> Result<TTSResponse, reqwest::Error> {
        let tts_res: TTSResponse = self
            .client
            .post(self.config.url)
            .headers(self.headers)
            .send()
            .await?
            .json::<TTSResponse>()
            .await?;

        Ok(tts_res)
    }

    pub async fn try_download(self, url: String) -> Result<String, DownloadError> {
        // perform lookup
        let res = self.client.get(&url).send().await?;
        let mut guard = self.url_map.lock().await;
        let value = guard.get(&url);
        match value {
            Some(word) => {
                let filename = format!("{}.mp3", word.replace(" ", "_"));
                let path = self.config.download_location.join(filename);
                let mut file = File::create(path)?;
                let response = reqwest::get(url).await?;
                let mut content = Cursor::new(response.bytes().await?);
                copy(&mut content, &mut file);
                Ok(word.to_string())
            }
            None => Err(DownloadError::NoWordFoundForURL()),
        }
    }
}
