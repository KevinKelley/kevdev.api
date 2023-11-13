pub mod models;
pub mod schema;
pub mod error_handler;

mod handler;
// mod model;
mod response;
mod route;
mod db;



use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use route::create_router;
use tower_http::cors::CorsLayer;

// use todo_api::*;


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


/////////////////////////////////////////////
// // Error handling with fallible handlers
// use axum::{
//    Router,
//    http::{Response, StatusCode},
//    error_handling::HandleError,
// };
// #[tokio::main]
// async fn main() {
//    // this service might fail with `reqwest::Error`
//    let fallible_service = tower::service_fn(|_req| async {
//      let body = can_fail().await?;
//      Ok::<_, reqwest::Error>(Response::new(body))
//    });
//    // Since fallible_service can fail with 'reqwest::Error',
//    // you can't directly route it to "/".
//    // Use route_service to convert any errors
//    // encountered into a response
//    let app = Router::new()
//      .route_service("/", HandleError::new(fallible_service, handle_error) );
  
//    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
//      .serve(app.into_make_service())
//      .await
//      .unwrap();
// }
// async fn can_fail() -> Result<String, reqwest::Error> {
//    // send a request to a site that doesn't exist 
//    // so we can see the handler fail
//    let body = reqwest::get("https://www.abcdth.org")
//      .await?
//      .text()
//      .await?;
//    Ok(body)
// }
// async fn handle_error(err: reqwest::Error) -> (StatusCode, String) {
//    return (
//        StatusCode::INTERNAL_SERVER_ERROR,
//        format!("Something went wrong: {}", err),
//    );
// }

/////////////////////////////////////////////
// Error Handling with pattern matching
// use axum::{
//  Router,
//  routing::get,
//  http::StatusCode,
// };

// #[tokio::main]
// async fn main() {

//    let app = Router::new()
//        .route("/", get(|| async {
//            match reqwest::get("https://www.abcdth.org").await {
//                Ok(res) => (
//                    StatusCode::OK,
//                    res.text().await.unwrap(),
//                ),
//                Err(err) => (
//                    StatusCode::INTERNAL_SERVER_ERROR,
//                    format!("Server failed with {}", err),
//                )
//            }
//        }));
  
//    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
//        .serve(app.into_make_service())
//        .await
//        .unwrap();
// }