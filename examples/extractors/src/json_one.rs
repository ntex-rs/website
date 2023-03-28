// <json-one>
use ntex::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
#[web::post("/submit")]
async fn submit(info: web::types::Json<Info>) -> Result<String, web::Error> {
    Ok(format!("Welcome {}!", info.username))
}
// </json-one>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(submit))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
