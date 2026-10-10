use ntex::web;

// <vh>
#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new()
            .service(
                web::scope("/")
                    .guard(web::guard::Host("www.rust-lang.org"))
                    .route(
                        "",
                        web::to(|| async { web::HttpResponse::Ok().body("www") }),
                    ),
            )
            .service(
                web::scope("/")
                    .guard(web::guard::Host("users.rust-lang.org"))
                    .route(
                        "",
                        web::to(|| async { web::HttpResponse::Ok().body("user") }),
                    ),
            )
            .route("/", web::to(|| async { web::HttpResponse::Ok().build() }))
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </vh>
