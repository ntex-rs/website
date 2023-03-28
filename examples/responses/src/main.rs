pub mod auto;
pub mod chunked;
pub mod identity;
pub mod identity_two;
pub mod json_resp;

// <builder>
use ntex::web;

async fn index() -> web::HttpResponse {
    web::HttpResponse::Ok()
        .content_type("text/plain")
        .set_header("X-Hdr", "sample")
        .body("data")
}
// </builder>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
