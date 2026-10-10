#![allow(dead_code)]

// <error-handler>
use ntex::http::header;
use ntex::{Ctx, Middleware, Service};
use ntex::web;

pub struct Error;

impl<S, St> Middleware<S, St> for Error {
    type Service = ErrorMiddleware<S>;

    fn create(&self, _state: &St, service: S) -> Self::Service {
        ErrorMiddleware { service }
    }
}

pub struct ErrorMiddleware<S> {
    service: S,
}

impl<S, St, ReqSt> Service<St, web::WebRequest<ReqSt>> for ErrorMiddleware<S>
where
    S: Service<St, web::WebRequest<ReqSt>, Res = web::WebResponse>,
{
    type Res = web::WebResponse;
    type Error = S::Error;

    ntex::forward_ready!(St, service);
    ntex::forward_shutdown!(St, service);

    async fn call(
        &self,
        req: web::WebRequest<ReqSt>,
        ctx: Ctx<'_, Self, St>,
    ) -> Result<Self::Res, Self::Error> {
        let mut res = ctx.call(&self.service, req).await?;
        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            res.headers_mut().insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("Error"),
            );
        }
        Ok(res)
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new()
            .middleware(Error)
            .service(web::resource("/").route(
                web::get().to(|| async { web::HttpResponse::InternalServerError().build() }),
            ))
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </error-handler>
