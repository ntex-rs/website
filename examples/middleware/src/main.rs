pub mod default_headers;
pub mod errorhandler;
pub mod logger;
pub mod user_sessions;

// <simple>
use ntex::{Ctx, Middleware, Service};
use ntex::web;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

impl<S, St> Middleware<S, St> for SayHi {
    type Service = SayHiMiddleware<S>;

    fn create(&self, _state: &St, service: S) -> Self::Service {
        SayHiMiddleware { service }
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, St, ReqSt> Service<St, web::WebRequest<ReqSt>> for SayHiMiddleware<S>
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
        println!("Hi from start. You requested: {}", req.path());
        let res = ctx.call(&self.service, req).await?;
        println!("Hi from response");
        Ok(res)
    }
}
// </simple>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new().middleware(SayHi).service(
            web::resource("/").to(|| async {
                "Hello, middleware! Check the console where the server is run."
            }),
        )
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
