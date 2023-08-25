use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn to_file(
  path: &str,
  json: &serde_json::Value,
) -> Result<(), Box<dyn Error>> {
  let mut file = File::create(path)?;
  let content = serde_json::to_string_pretty(json).unwrap();
  file.write_all(content.as_bytes())?;
  Ok(())
}
