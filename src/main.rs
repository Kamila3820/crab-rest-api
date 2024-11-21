pub mod config;
pub mod item;

use axum::{
    http::Method, routing::get, Router
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // build our application with a single route
    let app = Router::new()
        .layer(
            CorsLayer::new() 
                .allow_origin(Any) 
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::PUT,
                    Method::DELETE,
                ])
        )
        .route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is atarting on : {:?}", listener);

    axum::serve(listener, app).await.unwrap();
}
