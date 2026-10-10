use ntex::web;

pub mod app;
pub mod config;
pub mod scope;
pub mod state;
pub mod vh;
pub mod mutable_state;

// <multi>
#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new()
            .service(
                web::scope("/app1")
                    .route("/", web::to(|| async { web::HttpResponse::Ok().build() })),
            )
            .service(
                web::scope("/app2")
                    .route("/", web::to(|| async { web::HttpResponse::Ok().build() })),
            )
            .route("/", web::to(|| async { web::HttpResponse::Ok().build() }))
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </multi>
