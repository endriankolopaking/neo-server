mod api;
mod db;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  db::init().await?;
  let conn = db::regexp_conn()?;
  let conn = Arc::new(Mutex::new(conn));
  api::init(conn).await;
  Ok(())
}
