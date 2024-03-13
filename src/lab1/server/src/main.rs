use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::post, Router};
use regex::Regex;
use std::num::ParseIntError;

use serde::{Deserialize, Serialize};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", post(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Debug)]
enum AppError {
    JsonRejection(JsonRejection),
    InvalidNumber(ParseIntError),
}

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        Self::JsonRejection(value)
    }
}

impl From<ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidNumber(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            AppError::InvalidNumber(e) => (
                StatusCode::BAD_REQUEST,
                format!("Could not parse number: {e}"),
            ),
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

#[derive(Deserialize)]
struct GrepNumbersRequest {
    input: String,
}

#[derive(Serialize)]
struct GrepNumbersResponse {
    numbers: Vec<u64>,
}

async fn root(
    axum::Json(json): axum::Json<GrepNumbersRequest>,
) -> Result<AppJson<GrepNumbersResponse>, AppError> {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let mut numbers = Vec::new();

    for (_, [num]) in re.captures_iter(&json.input).map(|c| c.extract()) {
        numbers.push(num.parse::<u64>()?);
    }

    Ok(AppJson(GrepNumbersResponse { numbers }))
}
