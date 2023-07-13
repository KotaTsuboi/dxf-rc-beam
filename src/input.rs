use serde_derive::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize)]
pub struct TomlInput {
    pub beam_height: f64,
    pub beam_width: f64,
    pub rebar_diameter: f64,
    pub gap_between_rebar: f64,
    pub cover_depth: f64,
    pub num_rebar: NumRebar,
    pub layer_name: LayerName,
}

#[derive(Deserialize)]
pub struct NumRebar {
    pub top_1: u32,
    pub top_2: u32,
    pub top_3: u32,
    pub bottom_1: u32,
    pub bottom_2: u32,
    pub bottom_3: u32,
}

#[derive(Deserialize)]
pub struct LayerName {
    pub concrete: String,
    pub rebar: String,
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

pub fn read_input(file_path: &str) -> Result<TomlInput, Box<dyn Error>> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<TomlInput, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
