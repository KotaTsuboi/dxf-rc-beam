use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub input_file: String,

    #[arg(short, long)]
    pub output_file: String,
}
