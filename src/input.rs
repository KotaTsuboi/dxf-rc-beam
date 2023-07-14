use serde_derive::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize)]
pub struct RcBeamDrawing {
    beam_height: f64,
    beam_width: f64,
    rebar_diameter: f64,
    gap_between_rebar: Option<f64>,
    cover_depth: Option<f64>,
    num_rebar: NumRebar,
    layer_name: Option<LayerName>,
}

impl RcBeamDrawing {
    pub fn beam_height(&self) -> f64 {
        self.beam_height
    }

    pub fn beam_width(&self) -> f64 {
        self.beam_width
    }

    pub fn rebar_diameter(&self) -> f64 {
        self.rebar_diameter
    }

    pub fn gap_between_rebar(&self) -> f64 {
        self.gap_between_rebar.unwrap_or(80.0)
    }

    pub fn cover_depth(&self) -> f64 {
        self.cover_depth.unwrap_or(70.0)
    }

    pub fn num_rebar(&self) -> NumRebar {
        self.num_rebar.clone()
    }

    pub fn layer_name(&self) -> LayerName {
        self.layer_name.clone().unwrap_or(LayerName {
            concrete: Some("RC大梁".to_string()),
            rebar: Some("RC鉄筋".to_string()),
        })
    }
}

#[derive(Deserialize, Clone)]
pub struct NumRebar {
    top_1: Option<u32>,
    top_2: Option<u32>,
    top_3: Option<u32>,
    bottom_1: Option<u32>,
    bottom_2: Option<u32>,
    bottom_3: Option<u32>,
}

impl NumRebar {
    pub fn top_1(&self) -> u32 {
        self.top_1.unwrap_or(0)
    }

    pub fn top_2(&self) -> u32 {
        self.top_2.unwrap_or(0)
    }

    pub fn top_3(&self) -> u32 {
        self.top_3.unwrap_or(0)
    }

    pub fn bottom_1(&self) -> u32 {
        self.bottom_1.unwrap_or(0)
    }

    pub fn bottom_2(&self) -> u32 {
        self.bottom_2.unwrap_or(0)
    }

    pub fn bottom_3(&self) -> u32 {
        self.bottom_3.unwrap_or(0)
    }
}

#[derive(Deserialize, Clone)]
pub struct LayerName {
    concrete: Option<String>,
    rebar: Option<String>,
}

impl LayerName {
    pub fn concrete(&self) -> String {
        self.concrete
            .clone()
            .unwrap_or_else(|| "RC大梁".to_string())
    }

    pub fn rebar(&self) -> String {
        self.rebar.clone().unwrap_or_else(|| "RC鉄筋".to_string())
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

pub fn read_input(file_path: &str) -> Result<RcBeamDrawing, Box<dyn Error>> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<RcBeamDrawing, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
