use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct JsonScratchMetadata {
    pub agent: String,
    pub semver: String,
    pub vm: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct JsonScratchBlock {
    pub opcode: String,
    pub next: serde_json::Value,
    pub parent: serde_json::Value,
    pub inputs: HashMap<String, serde_json::Value>,
    pub fields: HashMap<String, Vec<String>>,
    pub shadow: bool,
    #[serde(alias = "topLevel")]
    pub top_level: bool,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct JsonScratchTarget {
    #[serde(alias = "isStage")]
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, Vec<serde_json::Value>>,
    pub lists: HashMap<String, Vec<serde_json::Value>>,
    pub broadcasts: HashMap<String, Vec<serde_json::Value>>,
    pub blocks: HashMap<String, JsonScratchBlock>,
    pub comments: HashMap<String, Vec<serde_json::Value>>,

    #[serde(alias = "currentCostume")]
    pub current_costume: i32,
    pub costumes: Vec<HashMap<String, serde_json::Value>>,
    pub sounds: Vec<serde_json::Value>,
    pub volume: i32,

    #[serde(alias = "layerOrder")]
    pub layer_order: i32,
    pub tempo: i32,

    #[serde(alias = "videoTransparency")]
    pub video_transparency: i32,

    #[serde(alias = "videoState")]
    pub video_state: String,

    #[serde(alias = "textToSpeechLanguage")]
    pub text_to_speech_language: serde_json::Value,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct JsonScratchFile {
    pub targets: Vec<JsonScratchTarget>,
    pub monitors: Vec<HashMap<String, serde_json::Value>>,
    pub extensions: Vec<String>,
    pub meta: JsonScratchMetadata,
}
