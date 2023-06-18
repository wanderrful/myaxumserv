mod routes;
mod models;
mod managers;
mod accessors;

use axum::{routing::{get, post}, http::StatusCode, response::IntoResponse, Json, Router, http};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::routes::api::ApiRouter;
use crate::routes::users::UserRouter;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .nest("/api", ApiRouter::new());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
