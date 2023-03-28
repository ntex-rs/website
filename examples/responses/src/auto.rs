// <auto>
use ntex::web;

#[web::get("/")]
async fn index() -> web::HttpResponse {
    web::HttpResponse::Ok().body("data")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .wrap(web::middleware::Compress::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </auto>
