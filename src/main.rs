use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    stdr_rcbeam::run()?;
    Ok(())
}
