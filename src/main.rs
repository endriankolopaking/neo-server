mod db;
mod query;

const URL: &str = "sqlite:database/data.db";
const CSV: &str = "database/data.csv";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let content = query::get_content().await?;
  query::generate_csv(content)?;
  match db::create(URL).await {
    Ok(()) => db::import_csv_to_sqlite(URL, CSV).await?,
    Err(err) => return Err(err),
  }
  Ok(())
}
