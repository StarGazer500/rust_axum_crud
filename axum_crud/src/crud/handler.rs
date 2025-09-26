// use axum::body::Body;

use crate::crud::services::service_sample;

pub async fn some_string_handler(body:String)->String{

    return service_sample(body).await
}