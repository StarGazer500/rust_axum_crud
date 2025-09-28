use axum::{
    extract::{State},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use crate::crud::services::{save_credentials_service,get_credentials_by_email_service};
// use crate::crud::model::ResponseCredentials;
use crate::crud::dto::{RequestCredentials,GetByEmailRequest};
use crate::grouped_routes::main_route::AppState;
use crate::crud::error_traits::{AppResult};

// #[axum::debug_handler]
// pub async fn save_credentials_handler(
//     State(state): State<AppState>,
//     Json(body): Json<RequestCredentials>
// ) -> impl IntoResponse {
//     match save_credentials_service(body, &state.db).await {
//         Ok(response) => (StatusCode::CREATED, Json(response)),
//         Err(sqlx::Error::Database(db_err)) if db_err.constraint() == Some("credentials_email_key") => {
//             (
//                 StatusCode::CONFLICT,
//                 Json(ResponseCredentials {
//                     email: "Email already exists".to_string(),
//                     password: String::new(),
//                 })
//             )
//         }
//         Err(e) => {
//             eprintln!("Database error: {:?}", e);
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(ResponseCredentials {
//                     email: "Internal server error".to_string(),
//                     password: String::new(),
//                 })
//             )
//         }
//     }
// }

// Updated Handlers - much cleaner now!
#[axum::debug_handler]
pub async fn save_credentials_handler(
    State(state): State<AppState>,
    Json(body): Json<RequestCredentials>,
) -> AppResult<impl IntoResponse> {
    let response = save_credentials_service(body, &state.db).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[axum::debug_handler]
pub async fn get_credentials_by_email_json_handler(
    State(state): State<AppState>,
    Json(body): Json<GetByEmailRequest>,
) -> AppResult<impl IntoResponse> {
    let credentials = get_credentials_by_email_service(&body.email, &state.db).await?;
    Ok((StatusCode::OK, Json(credentials)))
}


// #[axum::debug_handler]
// pub async fn get_credentials_by_email_json_handler(
//     State(state): State<AppState>,
//     Json(body): Json<GetByEmailRequest>
// ) -> impl IntoResponse {
//     match get_credentials_by_email_service(&body.email, &state.db).await {
//         Ok(Some(credentials)) => {
//             (StatusCode::OK, Json(credentials))
//         }
//         Ok(None) => {
//             (
//                 StatusCode::NOT_FOUND,
//                 Json(ResponseCredentials {
//                     email: "User not found".to_string(),
//                     password: String::new(),
//                 })
//             )
//         }
//         Err(e) => {
//             eprintln!("Database error: {:?}", e);
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(ResponseCredentials {
//                     email: "Internal server error".to_string(),
//                     password: String::new(),
//                 })
//             )
//         }
//     }
// }