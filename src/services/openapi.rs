use std::sync::Arc;

use ntex::web;
use ntex::http;
use ntex::util::Bytes;
use utoipa::OpenApi;

use crate::error::HttpError;
use crate::models::todo::{Todo, TodoPartial};

use super::todo;

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
  paths(
    todo::get_todos,
    todo::create_todo,
    todo::get_todo,
    todo::update_todo,
    todo::delete_todo,
  ),
  components(schemas(Todo, TodoPartial, HttpError))
)]
pub(crate) struct ApiDoc;

#[web::get("/{tail}*")]
async fn get_swagger(
  tail: web::types::Path<String>,
) -> Result<web::HttpResponse, HttpError> {
  if tail.as_ref() == "swagger.json" {
    let spec = ApiDoc::openapi().to_json().map_err(|err| HttpError {
      status: http::StatusCode::INTERNAL_SERVER_ERROR,
      msg: format!("Error generating OpenAPI spec: {}", err),
    })?;
    return Ok(
      web::HttpResponse::Ok()
        .content_type("application/json")
        .body(spec),
    );
  }
  let openapi_conf = Arc::new(
    utoipa_swagger_ui::Config::new(["/explorer/swagger.json"])
      .use_base_layout(),
  );
  match utoipa_swagger_ui::serve(&tail, openapi_conf).map_err(|err| {
    HttpError {
      msg: format!("Error serving Swagger UI: {}", err),
      status: http::StatusCode::INTERNAL_SERVER_ERROR,
    }
  })? {
    None => Err(HttpError {
      status: http::StatusCode::NOT_FOUND,
      msg: format!("path not found: {}", tail),
    }),
    Some(file) => Ok({
      let bytes = Bytes::from(file.bytes.to_vec());
      web::HttpResponse::Ok()
        .content_type(file.content_type)
        .body(bytes)
    }),
  }
}

pub fn ntex_config(config: &mut web::ServiceConfig<()>) {
  config.service(web::scope("/explorer/").service(get_swagger));
}
