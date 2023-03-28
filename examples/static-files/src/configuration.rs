// <config-one>
use ntex::web;
use ntex_files as fs;

#[web::get("/{filename}*")]
async fn index(req: web::HttpRequest) -> Result<fs::NamedFile, web::Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </config-one>
