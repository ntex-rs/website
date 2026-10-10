// <auto>
use ntex::web;

#[web::get("/")]
async fn index() -> web::HttpResponse {
    web::HttpResponse::Ok().body("data")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new()
            .middleware(web::middleware::Compress::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </auto>
