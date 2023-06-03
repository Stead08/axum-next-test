mod router;

use std::net::SocketAddr;
use crate::router::create_router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = create_router();
    let port = std::env::var("PORT").unwrap_or(String::from("8080"));
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
