use ntex::web;
use ntex::http;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

/// An http error response
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HttpError {
  /// The error message
  pub msg: String,
  /// The http status code, skipped in serialization
  #[serde(skip)]
  pub status: http::StatusCode,
}

/// Helper function to display an HttpError
impl std::fmt::Display for HttpError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.status, self.msg)
  }
}

/// Implement standard error for HttpError
impl std::error::Error for HttpError {}

/// Helper function to convert an HttpError into a ntex::web::HttpResponse
impl web::WebResponseError<(), web::DefaultError> for HttpError {
  fn error_response(&self, _: &()) -> web::HttpResponse {
    web::HttpResponse::builder(self.status).json(&self)
  }
}
