use axum::{
     Router
};

use crate::crud::routes::crud_routes;

pub  fn main_route()->Router{
    let crud_router = crud_routes();

    let api_routes = Router::new()
    .nest("/crud", crud_router);

    return api_routes;
   

}