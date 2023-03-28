// <resource>
use ntex::web;

async fn index() -> web::HttpResponse {
    web::HttpResponse::Ok().body("Hello")
}

pub fn main() {
    web::App::new()
        .service(web::resource("/prefix").to(index))
        .service(
            web::resource("/user/{name}")
                .name("user_detail")
                .guard(web::guard::Header("content-type", "application/json"))
                .route(web::get().to(|| async { web::HttpResponse::Ok().finish() }))
                .route(web::put().to(|| async { web::HttpResponse::Ok().finish() })),
        );
}
// </resource>
