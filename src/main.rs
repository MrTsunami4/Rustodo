#![warn(clippy::all)]
#![warn(clippy::nursery)]

mod db;
mod models;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    let app = routes::get_router().await;

    server::run_server(app).await;
}
