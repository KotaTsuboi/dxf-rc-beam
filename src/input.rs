use anyhow::Result;
use serde_derive::Deserialize;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize)]
pub struct RcBeamDrawing {
    pub beam_name: String,
    pub dimension: Dimension,
    pub main_rebar: MainRebar,
    pub stirrup: Stirrup,
    pub web_rebar: WebRebar,
    #[serde(default)]
    pub layer_name: LayerName,
    pub layout: Layout,
}

#[derive(Deserialize, Clone)]
pub struct Dimension {
    pub beam_width: f64,
    pub beam_height: f64,
    pub cover_depth: f64,
}

#[derive(Deserialize, Clone)]
pub struct MainRebar {
    pub diameter: f64,
    pub gap: f64,
    #[serde(default)]
    pub top_1: u32,
    #[serde(default)]
    pub top_2: u32,
    #[serde(default)]
    pub top_3: u32,
    #[serde(default)]
    pub bottom_1: u32,
    #[serde(default)]
    pub bottom_2: u32,
    #[serde(default)]
    pub bottom_3: u32,
}

#[derive(Deserialize, Clone)]
pub struct Stirrup {
    pub num: u32,
    pub diameter: f64,
    pub pitch: f64,
}

#[derive(Deserialize, Clone)]
pub struct WebRebar {
    #[serde(default)]
    pub diameter: f64,
    pub num_row: u32,
}

#[derive(Deserialize, Clone)]
pub struct LayerName {
    pub concrete: String,
    pub rebar: String,
    pub text: String,
}

#[derive(Deserialize, Clone)]
pub struct Layout {
    pub text_height: f64,
}

impl Default for LayerName {
    fn default() -> Self {
        LayerName {
            concrete: "RC大梁".to_string(),
            rebar: "RC鉄筋".to_string(),
            text: "注釈".to_string(),
        }
    }
}

fn read_file(path: &str) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(BufReader::new)
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn read_input(file_path: &str) -> Result<RcBeamDrawing> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<RcBeamDrawing, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
