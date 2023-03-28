pub mod cfg;
pub mod dhandler;
pub mod guard;
pub mod guard2;
pub mod minfo;
pub mod path;
pub mod path2;
pub mod resource;
pub mod scope;

// <main>
use ntex::web;

async fn index() -> web::HttpResponse {
    web::HttpResponse::Ok().body("Hello")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .route("/", web::get().to(index))
            .route("/user", web::post().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </main>
