use ntex::web;

use crate::models::todo::TodoPartial;

/// List all todos
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

/// Create a new todo
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

/// Get a todo by id
#[utoipa::path(
  get,
  path = "/todos/{id}",
  responses(
    (status = 200, description = "Todo found", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
#[web::get("/todos/{id}")]
pub async fn get_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// Update a todo by id
#[utoipa::path(
  put,
  path = "/todos/{id}",
  request_body = TodoPartial,
  responses(
    (status = 200, description = "Todo updated", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
#[web::put("/todos/{id}")]
pub async fn update_todo() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

/// Delete a todo by id
#[utoipa::path(
  delete,
  path = "/todos/{id}",
  responses(
    (status = 200, description = "Todo deleted", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
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
