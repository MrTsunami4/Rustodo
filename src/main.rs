#![warn(clippy::all)]
#![warn(clippy::nursery)]

mod auth;
mod db;
mod jwt;
mod models;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    let app = routes::get_router().await;

    server::run_server(app).await;
}
