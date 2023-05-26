use ntex::web;

mod error;
mod models;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      // Register swagger endpoints
      .configure(services::openapi::ntex_config)
      // Register todo endpoints
      .configure(services::todo::ntex_config)
      // Default endpoint for unregisterd endpoints
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;
  Ok(())
}
