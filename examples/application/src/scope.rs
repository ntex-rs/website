use ntex::web;

#[web::get("/show")]
async fn show_users(_req: web::HttpRequest) -> impl web::Responder {
    "unimplemented!"
}

// <scope>
#[ntex::main]
async fn main() {
    let _app = web::App::default().service(web::scope("/users").service(show_users));
}
// </scope>
