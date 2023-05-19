use std::fs::File;
use std::io::Write;

const KEY: &str = "1yRLGaQk3-9UlopftPr5e8F-X3pKkjwLlZWcTwai6_Ds";
const NAME: &str = "RRI+2.0+-+Masterlist";

pub async fn get_content() -> Result<String, Box<dyn std::error::Error>> {
  let url: String = format!(
    "https://docs.google.com/spreadsheets/d/{}/gviz/tq?tqx=out:csv&sheet={}",
    KEY, NAME
  );

  let response = reqwest::get(&url)
    .await?
    .text()
    .await?;

  Ok(response)
}

fn clean_csv(content: String) -> String {
  let mut lines: Vec<&str> = content
    .lines()
    .collect();

  if lines.len() >= 2 {
    lines.truncate(lines.len() - 2)
  } else {
    lines.clear();
  }

  lines.join("\n")
}

pub fn generate_csv(content: String) -> Result<(), Box<dyn std::error::Error>> {
  let mut file = File::create("database/data.csv")?;
  let content = clean_csv(content);
  file.write_all(content.as_bytes())?;
  Ok(())
}
