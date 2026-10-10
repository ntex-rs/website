use ntex::web;
use std::convert::Infallible;

// <path-three>
#[web::get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(req: web::HttpRequest) -> Result<String, Infallible> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().get("user_id").unwrap().parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().service(index))
        .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
        .run()
        .await
}
// </path-three>
