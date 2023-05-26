use ntex::web;

use crate::models::todo::TodoPartial;

#[utoipa::path(
  get,
  path = "/todos",
  responses(
    (status = 200, description = "List of Todo", body = [Todo]),
  ),
)]
#[web::get("/todos")]
pub async fn get_todos() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[utoipa::path(
  post,
  path = "/todos",
  request_body = TodoPartial,
  responses(
    (status = 201, description = "Todo created", body = Todo),
  ),
)]
#[web::post("/todos")]
pub async fn create_todo(
  _todo: web::types::Json<TodoPartial>,
) -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}

#[web::get("/todos/{id}")]
pub async fn get_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[web::put("/todos/{id}")]
pub async fn update_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[web::delete("/todos/{id}")]
pub async fn delete_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_todos);
  cfg.service(create_todo);
  cfg.service(get_todo);
  cfg.service(update_todo);
  cfg.service(delete_todo);
}
