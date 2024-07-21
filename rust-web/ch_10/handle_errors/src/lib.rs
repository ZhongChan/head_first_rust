use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

use argon2::Error as ArgonError;
use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as MiddlewareReqwestError;
use tracing::{event, instrument, Level};

const DUPLICATE_KYE: u32 = 23505;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    WrongPassword,
    CannotDecryptToken,
    Unauthorized,
    ArgonLibaryError(ArgonError),
    DatabaseQueryError(sqlx::Error),
    ReqwestAPIError(ReqwestError),
    MiddlewareReqwestAPIError(MiddlewareReqwestError),
    ClientError(APILayerError),
    SereverError(APILayerError),
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
            Error::DatabaseQueryError(_) => {
                write!(f, "Query could not be executed")
            }
            Error::ReqwestAPIError(e) => {
                write!(f, "Cannot execute: {}", e)
            }
            Error::ClientError(e) => {
                write!(f, "External Clien error: {}", e)
            }
            Error::SereverError(e) => {
                write!(f, "External API error: {}", e)
            }
            Error::MiddlewareReqwestAPIError(e) => {
                write!(f, "External API error: {}", e)
            }
            Error::WrongPassword => {
                write!(f, "Wrong password")
            }
            Error::ArgonLibaryError(_) => {
                write!(f, "Cannot verify password")
            }
            Error::CannotDecryptToken => {
                write!(f, "Cannot decrypt token")
            }
            Error::Unauthorized => {
                write!(f, "No permission to change the underlying resource")
            }
        }
    }
}

/// `marker trait`
/// `https://doc.rust-lang.org/std/marker/index.html`
/// `https://blog.rust-lang.org/2015/05/11/traits.html`
impl Reject for Error {}
impl Reject for APILayerError {}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::Unauthorized) = r.find() {
        event!(Level::ERROR, "Not matching account id");
        return Ok(warp::reply::with_status(
            "No permission to change the underlying resource".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    if let Some(Error::CannotDecryptToken) = r.find() {
        event!(Level::ERROR, "Cannot decrypt token");
        return Ok(warp::reply::with_status(
            "Cannot decrypt token".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    if let Some(Error::DatabaseQueryError(e)) = r.find() {
        event!(Level::ERROR, "Database query error");

        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KYE {
                    return Ok(warp::reply::with_status(
                        "Account already existsts".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ));
                }
                return Ok(warp::reply::with_status(
                    "Cannot update data".to_string(),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ));
            }
            _ => {
                return Ok(warp::reply::with_status(
                    "Cannot update data".to_string(),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ));
            }
        }
    }

    if let Some(Error::WrongPassword) = r.find() {
        event!(Level::ERROR, "Eneterd wrong password");
        return Ok(warp::reply::with_status(
            "Wrong E-Mail/Password combition".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    if let Some(Error::ReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        return Ok(warp::reply::with_status(
            "Internal Serever Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    if let Some(Error::MiddlewareReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        return Ok(warp::reply::with_status(
            "Internal Serever Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    if let Some(Error::ClientError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        return Ok(warp::reply::with_status(
            "Internal Serever Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    if let Some(Error::SereverError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        return Ok(warp::reply::with_status(
            "Internal Serever Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
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

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}
