use ntex::web;
use serde::Deserialize;

// pub mod custom_handler;
pub mod form;
pub mod json_one;
pub mod json_two;
pub mod multiple;
pub mod path_one;
pub mod path_three;
pub mod path_two;
pub mod query;

#[derive(Deserialize, Debug)]
struct MyInfo {
    username: String,
    id: u32,
}

// <option-one>
async fn index(
    path: web::types::Path<(String, String)>,
    json: web::types::Json<MyInfo>,
) -> impl web::Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}
// </option-one>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().route("/{name}/{id}", web::post().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
