mod accessors;
mod clients;
mod managers;
mod models;
mod resources;
mod routes;
mod utils;

use std::net::SocketAddr;

use axum::Router;

use crate::routes::api::ApiRouter;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", ApiRouter::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
