mod query;

use std::sync::{Arc, Mutex};
use warp::http::StatusCode;
use warp::Filter;

pub fn api(
  db: Arc<Mutex<rusqlite::Connection>>
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
  warp::post()
    .and(warp::path("api"))
    .and(warp::path("search"))
    .and(warp::body::json())
    .map(
      move |data: serde_json::Value| {
        let mut results: Vec<serde_json::Value> = Vec::new();

        if let serde_json::Value::Object(obj) = data {
          let query = query::generate(obj);
          // println!("{}", query);

          let db = db
            .lock()
            .expect("Query failed");

          let mut stmt = db
            .prepare(query.as_str())
            .expect("Error in query");

          let column_names: Vec<String> = stmt
            .column_names()
            // .unwrap()
            .into_iter()
            .map(String::from)
            .collect();

          let rows = stmt
            .query_map([], |row| {
              // row.get::<usize, String>(1)

              let mut json_obj = serde_json::json!({});
              for (i, col_name) in column_names
                .iter()
                .enumerate()
              {
                let col_value: String = row.get_unwrap::<usize, String>(i);
                json_obj[col_name] = serde_json::json!(col_value);
              }
              Ok(json_obj)
            })
            .expect("Failed to map row");

          // let mut results: Vec<String> = Vec::new();
          for row in rows {
            results.push(row.expect("Push failed"));
          }
        }

        let ret_msg = serde_json::json!({
          "count": results.len(),
          "results": results
        });

        println!(
          "Number of return {}",
          results.len()
        );

        warp::reply::with_status(
          warp::reply::json(&ret_msg),
          StatusCode::OK,
        )
      },
    )
}
