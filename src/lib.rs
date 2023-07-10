use clap::Parser;
use std::error::Error;

mod args;
mod input;
mod output;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    let input = input::read_input(&args.input_file)?;
    println!("height: {}, width: {}", input.beam_height, input.beam_width);
    output::write(input, &args.output_file)?;
    Ok(())
}
