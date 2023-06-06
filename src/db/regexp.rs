use regex::Regex;
use rusqlite::functions::FunctionFlags;
use rusqlite::{Connection, Error, Result};
use std::sync::Arc;
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub fn create_function(db: &Connection) -> Result<()> {
  db.create_scalar_function(
    "regexp",
    2,
    FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
    move |ctx| {
      assert_eq!(
        ctx.len(),
        2,
        "called with unexpected number of arguments"
      );
      let regexp: Arc<Regex> = ctx.get_or_create_aux(
        0,
        |vr| -> Result<_, BoxError> {
          Ok(Regex::new(
            vr.as_str()?,
          )?)
        },
      )?;
      let is_match = {
        let text = ctx
          .get_raw(1)
          .as_str()
          .map_err(|e| Error::UserFunctionError(e.into()))?;

        regexp.is_match(text)
      };

      Ok(is_match)
    },
  )
}
