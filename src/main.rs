use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dxf_rc_beam::run()?;
    Ok(())
}
