// <websockets>
use std::{cell::Cell, io, rc::Rc, time::Duration, time::Instant};

use futures::future::{select, Either};
use ntex::util::Bytes;
use ntex::web::{self, ws};
use ntex::{rt, time, Ctx, Service};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// WebSockets service, websocket sink is the service state
struct WsService {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Rc<Cell<Instant>>,
    /// Heartbeat task is started
    started: Cell<bool>,
}

impl Service<ws::WsSink, ws::Frame> for WsService {
    type Res = Option<ws::Message>;
    type Error = io::Error;

    async fn ready(&self, ctx: Ctx<'_, Self, ws::WsSink>) -> Result<(), Self::Error> {
        // start heartbeat task, `ctx.st()` is the websocket sink
        if !self.started.replace(true) {
            rt::spawn(heartbeat(self.hb.clone(), ctx.st().clone()));
        }
        Ok(())
    }

    // handle incoming websockets frames
    async fn call(
        &self,
        frame: ws::Frame,
        _: Ctx<'_, Self, ws::WsSink>,
    ) -> Result<Self::Res, Self::Error> {
        let item = match frame {
            // update heartbeat
            ws::Frame::Ping(msg) => {
                self.hb.set(Instant::now());
                Some(ws::Message::Pong(msg))
            }
            // update heartbeat
            ws::Frame::Pong(_) => {
                self.hb.set(Instant::now());
                None
            }
            // send message back
            ws::Frame::Text(text) => Some(ws::Message::Text(
                String::from_utf8_lossy(&text).as_ref().into(),
            )),
            ws::Frame::Binary(bin) => Some(ws::Message::Binary(bin)),
            // close connection
            ws::Frame::Close(reason) => Some(ws::Message::Close(reason)),
            // ignore other frames
            _ => None,
        };
        Ok(item)
    }
}

/// helper method that sends ping to client every heartbeat interval
async fn heartbeat(hb: Rc<Cell<Instant>>, sink: ws::WsSink) {
    loop {
        match select(Box::pin(time::sleep(HEARTBEAT_INTERVAL)), sink.on_disconnect()).await {
            Either::Left(_) => {
                // check client heartbeats
                if Instant::now().duration_since(hb.get()) > CLIENT_TIMEOUT {
                    // heartbeat timed out
                    println!("Websocket Client heartbeat failed, disconnecting!");
                    sink.io().close();
                    return;
                }

                // send ping
                if sink.send(ws::Message::Ping(Bytes::default())).await.is_err() {
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
async fn ws_index(req: web::HttpRequest) {
    // iterate over subprotocols requested by the client in the `Sec-Websocket-Protocol`
    // header and negotiate a chosen subprotocol as per the RFC 6455 Section 1.9
    let chosen = ws::subprotocols(&req).find(|p| *p == "my-subprotocol");

    // or simply ignore subprotocols and do not send `Sec-Websocket-Protocol` header in upgrade response
    // let chosen = None;

    let service = WsService {
        hb: Rc::new(Cell::new(Instant::now())),
        started: Cell::new(false),
    };

    // `start` sends the handshake response and runs the websocket
    // service until the connection is closed
    if let Err(err) = ws::start(&req, chosen, service).await {
        println!("WebSocket error: {:?}", err);
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server(async |_| {
        web::App::new()
            // enable logger
            .middleware(web::middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080", ntex::SharedCfg::default())?
    .run()
    .await
}
// </websockets>

// testing requires specific headers:
// Upgrade: websocket
// Connection: Upgrade
// Sec-WebSocket-Key: SOME_KEY
// Sec-WebSocket-Version: 13
