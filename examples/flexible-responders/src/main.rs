use ntex::web;
use serde::Serialize;

// <flexible-responders>
#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

async fn hello_world() -> impl web::Responder {
    "Hello World!"
}

async fn current_temperature() -> impl web::Responder {
    web::HttpResponse::Ok().json(&Measurement { temperature: 42.3 })
}
// </flexible-responders>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(web::resource("/").to(hello_world))
            .service(web::resource("/temp").to(current_temperature))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
