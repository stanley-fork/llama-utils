use hyper::{Body, Response};
use thiserror::Error;

#[allow(dead_code)]
pub(crate) fn not_implemented() -> Response<Body> {
    // log error
    error!(target: "stdout", "501 Not Implemented");

    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .status(hyper::StatusCode::NOT_IMPLEMENTED)
        .body(Body::from("501 Not Implemented"))
        .unwrap()
}

pub(crate) fn internal_server_error(msg: impl AsRef<str>) -> Response<Body> {
    let err_msg = match msg.as_ref().is_empty() {
        true => "500 Internal Server Error".to_string(),
        false => format!("500 Internal Server Error: {}", msg.as_ref()),
    };

    // log error
    error!(target: "stdout", "{}", &err_msg);

    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(err_msg))
        .unwrap()
}

pub(crate) fn bad_request(msg: impl AsRef<str>) -> Response<Body> {
    let err_msg = match msg.as_ref().is_empty() {
        true => "400 Bad Request".to_string(),
        false => format!("400 Bad Request: {}", msg.as_ref()),
    };

    // log error
    error!(target: "stdout", "{}", &err_msg);

    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .status(hyper::StatusCode::BAD_REQUEST)
        .body(Body::from(err_msg))
        .unwrap()
}

pub(crate) fn unauthorized(msg: impl AsRef<str>) -> Response<Body> {
    let err_msg = match msg.as_ref().is_empty() {
        true => "401 Unauthorized".to_string(),
        false => format!("401 Unauthorized: {}", msg.as_ref()),
    };

    // log error
    error!(target: "stdout", "{}", &err_msg);

    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .status(hyper::StatusCode::UNAUTHORIZED)
        .body(Body::from(err_msg))
        .unwrap()
}

pub(crate) fn invalid_endpoint(msg: impl AsRef<str>) -> Response<Body> {
    let err_msg = match msg.as_ref().is_empty() {
        true => "404 The requested service endpoint is not found".to_string(),
        false => format!(
            "404 The requested service endpoint is not found: {}",
            msg.as_ref()
        ),
    };

    // log error
    error!(target: "stdout", "{}", &err_msg);

    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .status(hyper::StatusCode::NOT_FOUND)
        .body(Body::from(err_msg))
        .unwrap()
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ServerError {
    /// Error returned while parsing CLI options failed
    #[error("{0}")]
    ArgumentError(String),
    /// Generic error returned while performing an operation
    #[error("{0}")]
    Operation(String),
}
