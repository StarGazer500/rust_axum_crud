use axum::{
    routing::{post},Router
};

use crate::crud::handler::some_string_handler;


pub fn crud_routes()->Router{
    let crud_app = Router::new()
    .route("/mirror_string", post(some_string_handler));
return  crud_app;

}