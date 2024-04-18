use clap::Parser;
use std::{error::Error, path::Path};

mod args;
mod input;
mod output;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    let input = input::read_input(&args.input_file)?;
    let output_file = remove_extension(&args.input_file);
    let output_file = format!("{output_file}.dxf");
    output::write(input, &output_file)?;
    Ok(())
}

fn remove_extension(file_name_with_extension: &str) -> &str {
    Path::new(file_name_with_extension)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
}
