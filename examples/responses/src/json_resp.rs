// <json-resp>
use ntex::web;
use std::convert::Infallible;
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[web::get("/a/{name}")]
async fn index(name: web::types::Path<String>) -> Result<impl web::Responder, Infallible> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::HttpResponse::Ok().json(&obj))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().service(index))
        .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
        .run()
        .await
}
// </json-resp>
