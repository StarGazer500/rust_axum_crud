use axum::{
    routing::post,
    Router
};
use crate::crud::handler::{save_credentials_handler,get_credentials_by_email_json_handler};
use crate::grouped_routes::main_route::AppState;

pub fn save_credential_crud_routes() -> Router<AppState> {
    Router::new()
      .route("/save_credentials", post(save_credentials_handler))
      .route("/get_by_email", post(get_credentials_by_email_json_handler))
}