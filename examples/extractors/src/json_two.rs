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
    web::HttpServer::new(async |_| {
        // change json extractor configuration
        let config =
            web::WebAppConfig::new().set_state(web::types::JsonConfig::default().limit(4096));

        web::App::new()
            .with_config(config)
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </json-two>
