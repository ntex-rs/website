// <path>
use ntex::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// extract path info using serde
#[web::get("/{username}/index.html")] // <- define path parameters
async fn index(info: web::types::Path<Info>) -> Result<String, web::Error> {
    Ok(format!("Welcome {}!", info.username))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </path>
