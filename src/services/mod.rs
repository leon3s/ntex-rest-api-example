pub mod todo;

use ntex::web;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}
