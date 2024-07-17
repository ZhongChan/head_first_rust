use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

use tracing::{event, instrument, Level};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseQueryError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => {
                write!(f, "Missing Parameter")
            }
            Error::DatabaseQueryError => {
                write!(f, "Query could not be executed")
            }
        }
    }
}

/// `marker trait`
/// `https://doc.rust-lang.org/std/marker/index.html`
/// `https://blog.rust-lang.org/2015/05/11/traits.html`
impl Reject for Error {}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError) = r.find() {
        event!(Level::ERROR, "Database query error");
        return Ok(warp::reply::with_status(
            Error::DatabaseQueryError.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    if let Some(error) = r.find::<Error>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ));
    }

    if let Some(error) = r.find::<BodyDeserializeError>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
