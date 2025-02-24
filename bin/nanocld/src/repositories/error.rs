use ntex::web;
use ntex::{http::StatusCode, web::error::BlockingError};

use crate::error::HttpError;

/// ## Db Error
///
/// Helper function to convert diesel::result::Error into HttpResponseError
/// With custom context
///
/// ## Arguments
///
/// - [context](str) - Custom context
///
/// ## Examples
///
/// ```rust,norun
/// use crate::repositories::errors::db_error;
///
/// let err = diesel::result::Error::NotFound;
/// let err = db_error("Custom context")(err);
/// assert_eq!(err.status, 404);
///```
///
pub fn db_error(
  context: &str,
) -> impl FnOnce(diesel::result::Error) -> HttpError {
  let context = context.to_owned();
  move |err: diesel::result::Error| -> HttpError {
    log::debug!("StoreError {context} {err}");
    let default_error = HttpError {
      msg: format!("StoreError {context} {err}"),
      status: StatusCode::BAD_REQUEST,
    };
    match err {
      diesel::result::Error::InvalidCString(_) => default_error,
      diesel::result::Error::DatabaseError(dberr, infoerr) => match dberr {
        diesel::result::DatabaseErrorKind::UniqueViolation => HttpError {
          msg: format!(
            "StoreError {context} {}",
            infoerr.details().unwrap_or_default()
          ),
          status: StatusCode::CONFLICT,
        },
        _ => default_error,
      },
      diesel::result::Error::NotFound => HttpError {
        msg: format!("StoreError {context} not found"),
        status: StatusCode::NOT_FOUND,
      },
      diesel::result::Error::QueryBuilderError(_) => default_error,
      diesel::result::Error::DeserializationError(_) => default_error,
      diesel::result::Error::SerializationError(_) => default_error,
      diesel::result::Error::RollbackTransaction => default_error,
      diesel::result::Error::AlreadyInTransaction => default_error,
      _ => HttpError {
        msg: format!("Unhandled error {context} {err:#}"),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      },
    }
  }
}

/// Convert BlockingError<diesel::result::Error> into HttpResponseError
///
/// # Arguments
///
/// * `err` - BlockingError diesel result error
///
/// # Examples
///
/// ```
/// // Return Error
///
/// use crate::repositories::errors::db_blocking_error;
/// Err(db_blocking_error(err))
/// ```
pub fn db_blocking_error(err: BlockingError<HttpError>) -> HttpError {
  match err {
    web::error::BlockingError::Error(err) => err,
    web::error::BlockingError::Canceled => HttpError {
      msg: String::from("Blocking error: Canceled"),
      status: StatusCode::INTERNAL_SERVER_ERROR,
    },
  }
}
