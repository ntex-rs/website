// <minfo>
use ntex::web;

#[web::get("/a/{v1}/{v2}/")]
async fn index(req: web::HttpRequest) -> Result<String, web::Error> {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();
    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
    Ok(format!("Values {} {} {} {}", v1, v2, v3, v4))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </minfo>
