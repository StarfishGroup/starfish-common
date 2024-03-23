use anyhow::{ensure, Result};
use snowflaked::sync::Generator;

pub fn init(seq: u16) -> Result<Generator> {
    ensure!(seq < 1024);
    Ok(Generator::new(seq))
}
