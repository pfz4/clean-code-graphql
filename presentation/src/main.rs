#![feature(is_some_with)]

use rocket::tokio;

mod graphapi;
mod server;

#[tokio::main]
pub async fn main() {
    //Launch Rocket Server
    let _ = server::launch()
    .await
    .unwrap();
}