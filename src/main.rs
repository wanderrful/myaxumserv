mod accessors;
mod clients;
mod managers;
mod middlewares;
mod models;
mod modules;
mod resources;
mod routes;
mod state;
mod utils;

use std::net::SocketAddr;

use axum::Router;

use crate::routes::api::ApiRouter;
use crate::middlewares::logger::LoggerMiddleware;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", ApiRouter::new())
        .layer(LoggerMiddleware);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
