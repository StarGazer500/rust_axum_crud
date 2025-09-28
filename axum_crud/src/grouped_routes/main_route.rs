use axum::{
     Router
};

use crate::crud::routes::save_credential_crud_routes;
use tower_http::cors::{CorsLayer};
use axum::http::{Method, HeaderValue,header};

use sqlx::{PgPool};
#[derive(Clone)]
pub struct AppState {
   pub db: PgPool,
}
pub fn main_route(state: AppState) -> Router {
    let crud_router = save_credential_crud_routes();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()) // Vite dev server
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_methods([
            Method::GET, 
            Method::POST, 
            Method::PUT, 
            Method::DELETE, 
            Method::OPTIONS
        ])
        .allow_headers([
            header::AUTHORIZATION,     // For Bearer tokens
            header::CONTENT_TYPE,     // For JSON requests
            header::ACCEPT,           // Standard accept header
        ])
        .allow_credentials(true);

    let api_routes = Router::new()
        .nest("/crud", crud_router)
        .layer(cors)
        .with_state(state); // Apply state here
    api_routes
}