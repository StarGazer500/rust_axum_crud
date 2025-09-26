mod grouped_routes;
mod crud;

use crate::grouped_routes::main_route::{main_route};

pub async fn run(){
    tracing_subscriber::fmt::init();
    let app=main_route();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


}