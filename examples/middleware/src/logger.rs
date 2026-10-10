// <logger>
use env_logger::Env;
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    web::HttpServer::new(async |_| {
        web::App::new()
            .middleware(web::middleware::Logger::default())
            .middleware(web::middleware::Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(async || "Hello"))
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </logger>
