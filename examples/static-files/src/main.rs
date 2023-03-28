pub mod configuration;
pub mod configuration_two;
pub mod directory;

// <individual-file>
use ntex::web;
use ntex_files::NamedFile;
use std::path::PathBuf;

async fn index(req: web::HttpRequest) -> Result<NamedFile, web::Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().route("/{filename}*", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </individual-file>
