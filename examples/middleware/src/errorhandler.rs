#![allow(dead_code)]

use futures_util::Future;
// <error-handler>
use ntex::http::header;
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;

pub struct Error;

impl<S> Middleware<S> for Error {
    type Service = ErrorMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        ErrorMiddleware { service }
    }
}

pub struct ErrorMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for ErrorMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_poll_ready!(service);

    fn call(&self, req: web::WebRequest<Err>, ctx: ServiceCtx<Self>) -> impl Future<Output = Result<Self::Response, Self::Error>> {
        Box::pin(async move {
            ctx.call(&self.service, req).await.map(|mut res| {
                let status = res.status();
                if status.is_client_error() || status.is_server_error() {
                    res.headers_mut().insert(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_static("Error"),
                    );
                }
                res
            })
        })
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .wrap(Error)
            .service(web::resource("/").route(
                web::get().to(|| async { web::HttpResponse::InternalServerError().finish() }),
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </error-handler>
