#![allow(dead_code)]

// <example>
use ntex::web;

async fn index(_req: web::HttpRequest) -> web::HttpResponse {
    let mut resp = web::HttpResponse::Ok()
        .force_close() // <- Close connection on HttpResponseBuilder
        .finish();

    // Alternatively close connection on the HttpResponse struct
    resp.head_mut()
        .set_connection_type(ntex::http::ConnectionType::Close);

    resp
}
// </example>

// ConnectionType::Close
// ConnectionType::KeepAlive
// ConnectionType::Upgrade
