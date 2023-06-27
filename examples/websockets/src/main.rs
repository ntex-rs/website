// <websockets>
use std::{cell::RefCell, io, rc::Rc, time::Duration, time::Instant};

use ntex::web;
use ntex::util::Bytes;
use ntex::{fn_service, chain};
use ntex::{channel::oneshot, rt, time};
use futures::future::{ready, select, Either};
use ntex::service::{fn_factory_with_config, fn_shutdown, Service};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct WsState {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

/// WebSockets service factory
async fn ws_service(
    sink: web::ws::WsSink,
) -> Result<
    impl Service<web::ws::Frame, Response = Option<web::ws::Message>, Error = io::Error>,
    web::Error,
> {
    let state = Rc::new(RefCell::new(WsState { hb: Instant::now() }));

    // disconnect notification
    let (tx, rx) = oneshot::channel();

    // start heartbeat task
    rt::spawn(heartbeat(state.clone(), sink, rx));

    // handler service for incoming websockets frames
    let service = fn_service(move |frame| {
        let item = match frame {
            // update heartbeat
            web::ws::Frame::Ping(msg) => {
                state.borrow_mut().hb = Instant::now();
                Some(web::ws::Message::Pong(msg))
            }
            // update heartbeat
            web::ws::Frame::Pong(_) => {
                state.borrow_mut().hb = Instant::now();
                None
            }
            // send message back
            web::ws::Frame::Text(text) => Some(web::ws::Message::Text(
                String::from_utf8(Vec::from(text.as_ref())).unwrap().into(),
            )),
            web::ws::Frame::Binary(bin) => Some(web::ws::Message::Binary(bin)),
            // close connection
            web::ws::Frame::Close(reason) => Some(web::ws::Message::Close(reason)),
            // ignore other frames
            _ => None,
        };
        ready(Ok(item))
    });

    // handler service for shutdown notification that stop heartbeat task
    let on_shutdown = fn_shutdown(move || {
        let _ = tx.send(());
    });

    // pipe our service with on_shutdown callback
    Ok(chain(service).and_then(on_shutdown))
}

/// helper method that sends ping to client every heartbeat interval
async fn heartbeat(
    state: Rc<RefCell<WsState>>,
    sink: web::ws::WsSink,
    mut rx: oneshot::Receiver<()>,
) {
    loop {
        match select(Box::pin(time::sleep(HEARTBEAT_INTERVAL)), &mut rx).await {
            Either::Left(_) => {
                // check client heartbeats
                if Instant::now().duration_since(state.borrow().hb) > CLIENT_TIMEOUT {
                    // heartbeat timed out
                    println!("Websocket Client heartbeat failed, disconnecting!");
                    return;
                }

                // send ping
                if sink
                    .send(web::ws::Message::Ping(Bytes::default()))
                    .await
                    .is_err()
                {
                    return;
                }
            }
            Either::Right(_) => {
                println!("Connection is dropped, stop heartbeat task");
                return;
            }
        }
    }
}

/// do websocket handshake and start web sockets service
async fn ws_index(req: web::HttpRequest) -> Result<web::HttpResponse, web::Error> {
    web::ws::start(req, fn_factory_with_config(ws_service)).await
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server(|| {
        web::App::new()
            // enable logger
            .wrap(web::middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </websockets>

// testing requires specific headers:
// Upgrade: websocket
// Connection: Upgrade
// Sec-WebSocket-Key: SOME_KEY
// Sec-WebSocket-Version: 13
