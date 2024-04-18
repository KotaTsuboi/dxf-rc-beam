use anyhow::Result;

fn main() -> Result<()> {
    stdr_rcbeam::run()?;
    Ok(())
}
