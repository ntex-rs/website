#![allow(dead_code)]

// <config>
use ntex::web;

// this function could be located in a different module
fn scoped_config(cfg: &mut web::ServiceConfig<()>) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { web::HttpResponse::Ok().body("test") }))
            .route(web::head().to(|| async { web::HttpResponse::MethodNotAllowed().build() })),
    );
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig<()>) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { web::HttpResponse::Ok().body("app") }))
            .route(web::head().to(|| async { web::HttpResponse::MethodNotAllowed().build() })),
    );
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route(
                "/",
                web::get().to(|| async { web::HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </config>
