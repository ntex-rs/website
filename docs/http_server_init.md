---
title: HTTP Server Initialization
---

# Architecture overview

Below is a diagram of HttpServer initialization, which happens on the following code

```rust
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .route("/", web::to(|| async { web::HttpResponse::Ok().finish() }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

![](/img/diagrams/http_server.svg)
