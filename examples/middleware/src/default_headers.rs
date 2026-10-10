// <default-headers>
use ntex::http;
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new()
            .middleware(web::middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .service(
                web::resource("/test")
                    .route(web::get().to(|| async { web::HttpResponse::Ok().build() }))
                    .route(
                        web::method(http::Method::HEAD)
                            .to(|| async { web::HttpResponse::MethodNotAllowed().build() }),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </default-headers>
