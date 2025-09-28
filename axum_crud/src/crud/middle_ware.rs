// pub async fn error_logging_middleware<B>(
//     request: axum::http::Request<B>,
//     next: axum::middleware::Next<B>,
// ) -> axum::response::Response {
//     let response = next.run(request).await;
    
//     // Log if response is an error
//     if response.status().is_client_error() || response.status().is_server_error() {
//         tracing::warn!(
//             status = %response.status(),
//             "Request resulted in error"
//         );
//     }
    
//     response
// }