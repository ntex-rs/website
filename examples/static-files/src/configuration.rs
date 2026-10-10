// <config-one>
use ntex::web;
use ntex_files as fs;

#[web::get("/{filename}*")]
async fn index(req: web::HttpRequest) -> Result<fs::NamedFile, std::io::Error> {
    let path: std::path::PathBuf = req.match_info().get("filename").unwrap().parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().service(index))
        .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
        .run()
        .await
}
// </config-one>
