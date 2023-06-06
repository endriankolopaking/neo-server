pub mod search;
use warp::Filter;

use std::sync::{Arc, Mutex};

pub async fn init(db: Arc<Mutex<rusqlite::Connection>>) {
  let cors = warp::cors()
    .allow_any_origin()
    .allow_methods(vec!["GET", "POST"])
    .allow_headers(vec!["content-type"])
    .build();

  let api = search::api(db.clone());

  warp::serve(api.with(cors))
    .run(([0, 0, 0, 0], 8000))
    .await;
}
