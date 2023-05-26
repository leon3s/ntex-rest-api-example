use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

/// Todo model
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Todo {
  /// The todo id
  pub id: i32,
  /// The todo title
  pub title: String,
  /// The todo completed status
  pub completed: bool,
}

/// Partial Todo model
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoPartial {
  /// The todo title
  pub title: String,
}
