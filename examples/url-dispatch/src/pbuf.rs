// <pbuf>
use ntex::web;
use std::convert::Infallible;
use std::path::PathBuf;

#[web::get("/a/{tail}*")]
async fn index(req: web::HttpRequest) -> Result<String, Infallible> {
    let path: PathBuf = req.match_info().get("tail").unwrap().parse().unwrap();
    Ok(format!("Path {:?}", path))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().service(index))
        .bind("127.0.0.1:8080", ntex::SharedCfg::default())?
        .run()
        .await
}
// </pbuf>
