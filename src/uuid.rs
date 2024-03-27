use anyhow::{ensure, Result};
use snowflaked::sync::Generator;

#[derive(Debug)]
pub struct UUID(Generator);

impl UUID {
	pub fn take(&self) -> u64 {
		self.0.generate()
	}
}

pub fn init(seq: u16) -> Result<UUID> {
	ensure!(seq < 1024, "seq range : 0 - 1023");
	Ok(UUID(Generator::new(seq)))
}
