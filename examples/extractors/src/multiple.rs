// <multi>
use ntex::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[web::get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(
    path: web::types::Path<(u32, String)>,
    query: web::types::Query<Info>,
) -> String {
    let (user_id, friend) = path.into_inner();
    format!(
        "Welcome {}, friend {}, user_id {}!",
        query.username, friend, user_id
    )
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </multi>
