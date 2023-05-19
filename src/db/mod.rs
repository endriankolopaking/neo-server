use csv::ReaderBuilder;
use sqlx::{migrate::MigrateDatabase, Connection, Sqlite, SqliteConnection};
use std::error::Error;
use std::fs::File;

pub async fn create(url: &str) -> Result<(), Box<dyn Error>> {
  if Sqlite::database_exists(url).await? {
    println!("Database exist");
    match sqlx::Sqlite::drop_database(url).await {
      Ok(()) => {
        Sqlite::create_database(url).await?;
        println!("Database replaced");
      }
      Err(err) => {
        println!("Error {}", err)
      }
    }
  } else {
    println!("Database created");
    Sqlite::create_database(url).await?;
  }

  Ok(())
}

pub async fn import_csv_to_sqlite(
  db_url: &str,
  csv_path: &str,
) -> Result<(), Box<dyn Error>> {
  let mut connection = SqliteConnection::connect(db_url).await?;
  let file = File::open(csv_path)?;
  let mut reader = ReaderBuilder::new().from_reader(file);

  let headers = reader
    .headers()?
    .iter()
    .map(|s| format!(r#""{}""#, s))
    .collect::<Vec<String>>()
    .join(",");

  sqlx::query(
    format!(
      "CREATE TABLE data ({})",
      headers
    )
    .as_str(),
  )
  .execute(&mut connection)
  .await?;

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

    sqlx::query(
      format!("INSERT INTO data ({headers}) VALUES ({record})").as_str(),
    )
    .execute(&mut connection)
    .await?;
  }

  connection
    .close()
    .await?;
  Ok(())
}
