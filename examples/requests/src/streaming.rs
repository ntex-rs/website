// <streaming>
use futures::StreamExt;
use ntex::util::BytesMut;
use ntex::web;

#[web::get("/")]
async fn index(mut body: web::types::Payload) -> Result<web::HttpResponse, web::Error> {
    let mut bytes = BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    Ok(web::HttpResponse::Ok().finish())
}
// </streaming>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
