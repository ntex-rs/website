// <pbuf>
use ntex::web;
use std::path::PathBuf;

#[web::get("/a/{tail}*")]
async fn index(req: web::HttpRequest) -> Result<String, web::Error> {
    let path: PathBuf = req.match_info().query("tail").parse().unwrap();
    Ok(format!("Path {:?}", path))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
// </pbuf>
