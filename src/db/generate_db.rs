use csv::ReaderBuilder;
use rusqlite::{Connection, Result};
use std::error::Error;
use std::fs::File;

pub fn init(
  url: &str,
  csv: &str,
) -> Result<(), Box<dyn Error>> {
  let db = Connection::open(url)?;

  let test_db = match db.query_row(
    "SELECT * FROM data LIMIT 1",
    (),
    |row| row.get::<usize, String>(1),
  ) {
    Ok(value) => value,
    Err(err) => {
      println!("test_db : {}", err);
      "".to_string()
    }
  };

  if test_db == "" {
    import_csv_to_sqlite(url, csv)?;
  };

  Ok(())
}

pub fn import_csv_to_sqlite(
  db_url: &str,
  csv_path: &str,
) -> Result<(), Box<dyn Error>> {
  let connection = rusqlite::Connection::open(db_url)?;
  let file = File::open(csv_path)?;
  let mut reader = ReaderBuilder::new().from_reader(file);

  let headers = reader
    .headers()?
    .iter()
    .map(|s| format!(r#""{}""#, s))
    .collect::<Vec<String>>()
    .join(",");

  connection.execute(
    format!(
      "CREATE TABLE data ({})",
      headers
    )
    .as_str(),
    (),
  )?;

  while let Some(result) = reader
    .records()
    .next()
  {
    let record = result?
      .iter()
      .map(|s| s.replace("\"", r#""""#))
      .map(|s| format!(r#""{}""#, s))
      .collect::<Vec<String>>()
      .join(",");

    connection.execute(
      format!("INSERT INTO data ({headers}) VALUES ({record})").as_str(),
      (),
    )?;
  }

  println!(
    "Last inserted RowID: {:?}",
    connection.last_insert_rowid()
  );

  Ok(())
}
