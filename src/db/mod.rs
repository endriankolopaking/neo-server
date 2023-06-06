mod generate_db;
mod get_csv_data;
mod regexp;

use rusqlite::Connection;

const URL: &str = "database/data.db";
const CSV: &str = "database/data.csv";

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
  get_csv_data::fetch(CSV).await?;
  generate_db::init(URL, CSV)?;
  Ok(())
}

pub fn regexp_conn() -> Result<Connection, Box<dyn std::error::Error>> {
  let conn = Connection::open(URL)?;
  regexp::create_function(&conn)?;
  Ok(conn)
}
