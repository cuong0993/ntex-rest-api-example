use ntex::{web, SharedCfg};

mod error;
mod models;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(async |_| {
    web::App::new()
      .configure(|config| {
        // Register swagger and todo endpoints
        services::openapi::ntex_config(config);
        services::todo::ntex_config(config);
      })
      // Default endpoint for unregisterd endpoints
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080), SharedCfg::default())?
  .run()
  .await?;
  Ok(())
}
