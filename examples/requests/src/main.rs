pub mod manual;
pub mod multipart;
pub mod streaming;
pub mod urlencoded;

// <json-request>
use ntex::web;
use std::convert::Infallible;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
async fn index(info: web::types::Json<Info>) -> Result<String, Infallible> {
    Ok(format!("Welcome {}!", info.username))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().route("/", web::post().to(index)))
        .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
        .run()
        .await
}
// </json-request>
