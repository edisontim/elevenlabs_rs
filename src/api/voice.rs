use crate::{api::ClientBuilder, error::Error, prelude::*};
use comparable::*;
use http_body_util::{Empty, Full};
use hyper::body::Bytes;
use serde::{Deserialize, Serialize};

const GET: &str = "GET";
const BASE_PATH: &str = "/voices";
const SETTINGS_PATH: &str = "/settings";
const DEFAULT_SETTINGS_PATH: &str = "/voices/settings/default";
const EDIT_PATH: &str = "/edit";

#[derive(Debug, Serialize, Deserialize, Clone, Comparable)]
pub struct Voices {
    pub voices: Vec<Voice>,
}

impl Voices {
    pub fn all_clones(&self) -> Vec<&Voice> {
        self.voices
            .iter()
            .filter(|v| v.category == Some("cloned".to_string()))
            .collect::<Vec<&Voice>>()
    }
    pub fn by_name(&self, name: &str) -> Result<&Voice> {
        self.voices
            .iter()
            .find(|v| v.name == Some(name.to_string()))
            .ok_or(Box::new(Error::VoiceNotFound))
    }
    pub async fn get_voice_with_settings(&self, voice_name: &str) -> Result<Voice> {
        let voice = self.by_name(voice_name)?;
        let voice = get_voice(&voice.voice_id, true).await?;
        Ok(voice)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Comparable)]
pub struct Voice {
    pub voice_id: String,
    pub name: Option<String>,
    pub samples: Option<Vec<VoiceSample>>,
    pub category: Option<String>,
    pub labels: Option<Labels>,
    pub description: Option<String>,
    pub preview_url: Option<String>,
    pub settings: Option<VoiceSettings>,
}

impl Voice {
    pub async fn with_settings(voice_name: &str) -> Result<Self> {
        let voices = get_voices().await?;
        let voice = voices.by_name(voice_name)?;
        let voice = get_voice(&voice.voice_id, true).await?;
        Ok(voice)
    }
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, PartialEq)]
pub struct VoiceSample {
    pub sample_id: String,
    file_name: String,
    mime_type: String,
    size_bytes: Option<i64>,
    hash: String,
}

#[derive(Serialize, Deserialize, Comparable, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    additional_prop1: Option<String>,
    additional_prop2: Option<String>,
    additional_prop3: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Comparable)]
pub struct VoiceSettings {
    pub similarity_boost: f64,
    pub stability: f64,
    pub style: f64,
    pub use_speaker_boost: bool,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        VoiceSettings {
            similarity_boost: 0.75,
            stability: 0.5,
            style: 0.0,
            use_speaker_boost: true,
        }
    }
}

pub async fn get_voices() -> Result<Voices> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .method(GET)?
        .path(BASE_PATH)?
        .header("ACCEPT", "application/json")?
        .build()?;
    let resp = c.send_request(Empty::<Bytes>::new()).await?;
    let voices: Voices = serde_json::from_slice(&resp)?;
    Ok(voices)
}

pub async fn get_default_settings() -> Result<VoiceSettings> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .method(GET)?
        .path(DEFAULT_SETTINGS_PATH)?
        .header("ACCEPT", "application/json")?
        .build()?;
    let resp = c.send_request(Empty::<Bytes>::new()).await?;
    let voices_settings = serde_json::from_slice::<VoiceSettings>(&resp)?;
    Ok(voices_settings)
}

/// Get the voice settings for a specific voice
///
/// # Example
///```
///  use elevenlabs_rs::api::voice::{get_voice_settings, get_voices};
///  use elevenlabs_rs::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let v = get_voices().await?;
///     let cloned_voices = v.all_clones();
///     let settings = get_voice_settings(&cloned_voices[0].voice_id).await?;
///     println!("Settings: {:#?}", settings);
///     Ok(())
/// }
///
/// // prints:
///
/// // Settings: VoiceSettings {
/// //     similarity_boost: 0.125,
/// //     stability: 0.325,
/// //     style: 0.0,
/// //     use_speaker_boost: false,
/// // }
/// ```
pub async fn get_voice_settings(voice_id: &str) -> Result<VoiceSettings> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .method(GET)?
        .path(format!("{}/{}{}", BASE_PATH, voice_id, SETTINGS_PATH))?
        .header("ACCEPT", "application/json")?
        .build()?;
    let resp = c.send_request(Empty::<Bytes>::new()).await?;
    let voices_settings = serde_json::from_slice::<VoiceSettings>(&resp)?;
    Ok(voices_settings)
}

pub async fn get_voice(voice_id: &str, with_settings: bool) -> Result<Voice> {
    let path = if with_settings {
        format!("{}/{}?with_settings=true", BASE_PATH, voice_id)
    } else {
        format!("{}/{}", BASE_PATH, voice_id)
    };
    let cb = ClientBuilder::new()?;
    let c = cb
        .method(GET)?
        .path(path)?
        .header("ACCEPT", "application/json")?
        .build()?;
    let resp = c.send_request(Empty::<Bytes>::new()).await?;
    let voice = serde_json::from_slice::<Voice>(&resp)?;
    Ok(voice)
}

pub async fn delete_voice(voice_id: &str) -> Result<()> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .method("DELETE")?
        .path(format!("{}/{}", BASE_PATH, voice_id))?
        .header("ACCEPT", "application/json")?
        .build()?;
    let _resp = c.send_request(Empty::<Bytes>::new()).await?;
    Ok(())
}

/// Edit the voice settings for a specific voice
/// # Example
/// ```
/// use elevenlabs_rs::api::voice::*;
/// use elevenlabs_rs::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///    let voices = get_voices().await?;
///    let cloned_voices = voices.all_clones();
///    let voice = get_voice_settings(&cloned_voices[0].voice_id).await?;
///
///    let settings = VoiceSettings {
///         similarity_boost: 0.125,
///         stability: 0.325,
///         style: 0.0,
///         use_speaker_boost: true,
///    };
///
///    edit_voice_settings(&voice[0].voice_id, settings).await?;
///
///    Ok(())
/// }
/// ```
pub async fn edit_voice_settings(voice_id: &str, settings: VoiceSettings) -> Result<()> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .method("POST")?
        .path(format!(
            "{}/{}{}{}",
            BASE_PATH, voice_id, SETTINGS_PATH, EDIT_PATH
        ))?
        .header("ACCEPT", "application/json")?
        .build()?;
    let _resp = c
        .send_request(Full::<Bytes>::new(serde_json::to_string(&settings)?.into()))
        .await?;
    Ok(())
}