// <json-manual>
use futures::StreamExt;
use ntex::util::BytesMut;
use ntex::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[web::post("/")]
async fn index_manual(
    mut payload: web::types::Payload,
) -> Result<web::HttpResponse, web::Error> {
    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(web::error::ErrorBadRequest("overflow").into());
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(web::HttpResponse::Ok().json(&obj)) // <- send response
}
// </json-manual>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index_manual))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
