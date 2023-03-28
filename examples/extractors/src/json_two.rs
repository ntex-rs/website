#![allow(dead_code)]

// <json-two>
use ntex::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index(info: web::types::Json<Info>) -> impl web::Responder {
    format!("Welcome {}!", info.username)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        let json_config = web::types::JsonConfig::default().limit(4096);

        web::App::new().service(
            web::resource("/")
                // change json extractor configuration
                .state(json_config)
                .route(web::post().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </json-two>
