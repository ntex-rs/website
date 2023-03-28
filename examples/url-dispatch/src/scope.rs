use ntex::web;

// <scope>
#[web::get("/show")]
async fn show_users() -> web::HttpResponse {
    web::HttpResponse::Ok().body("Show users")
}

#[web::get("/show/{id}")]
async fn user_detail(path: web::types::Path<(u32,)>) -> web::HttpResponse {
    web::HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new().service(
            web::scope("/users")
                .service(show_users)
                .service(user_detail),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </scope>
