// <logger>
use env_logger::Env;
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    web::HttpServer::new(|| {
        web::App::new()
            .wrap(web::middleware::Logger::default())
            .wrap(web::middleware::Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </logger>
