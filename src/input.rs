use anyhow::Result;
use serde_derive::Deserialize;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize)]
pub struct RcBeamDrawing {
    pub beam_width: f64,
    pub beam_height: f64,
    pub rebar_diameter: f64,
    pub gap_between_rebar: f64,
    pub cover_depth: f64,
    pub num_rebar: NumRebar,
    #[serde(default)]
    pub layer_name: LayerName,
}

#[derive(Deserialize, Clone)]
pub struct NumRebar {
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
    #[serde(default)]
    pub side_rebar_row: u32,
}

#[derive(Deserialize, Clone)]
pub struct LayerName {
    pub concrete: String,
    pub rebar: String,
}

impl Default for LayerName {
    fn default() -> Self {
        LayerName {
            concrete: "RC大梁".to_string(),
            rebar: "RC鉄筋".to_string(),
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
