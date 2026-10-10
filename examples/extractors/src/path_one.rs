// <path-one>
use ntex::web;
use std::convert::Infallible;

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[web::get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(path: web::types::Path<(u32, String)>) -> Result<String, Infallible> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().service(index))
        .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
        .run()
        .await
}
// </path-one>
