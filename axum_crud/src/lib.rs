mod grouped_routes;
mod crud;
mod database;

use crate::grouped_routes::main_route::{main_route,AppState};
use crate::database::dbconnect::connect;


pub async fn run(){
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let pool = connect().await.unwrap();
     let app_state = AppState { db: pool };
    
    let app=main_route(app_state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


}