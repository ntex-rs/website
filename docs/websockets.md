---
title: Websockets
---

import CodeBlock from "@site/src/components/code_block.js";

# Websockets

ntex supports WebSockets out of the box. The [`web::ws::start()`][wsstart] function performs the websocket handshake, sends the handshake response and runs a websocket service until the connection is closed. A websocket service is a regular ntex [_Service_][service] that accepts [_ws::Frame_][frame] and optionally returns a [_ws::Message_][message] response. The service state is a [_ws::WsSink_][wssink], which can be used to send messages to the peer at any time. The handler that calls `start()` writes the response itself, so it returns `()`.

The following is an example of a simple websocket echo server:

<CodeBlock example="websockets" file="main.rs" section="websockets" />

> A simple websocket echo server example is available in the [examples directory][examples].

> An example chat server with the ability to chat over a websocket or TCP connection is available in [websocket-chat directory][chat]

[message]: https://docs.rs/ntex/latest/ntex/ws/enum.Message.html
[frame]: https://docs.rs/ntex/latest/ntex/ws/enum.Frame.html
[wssink]: https://docs.rs/ntex/latest/ntex/ws/struct.WsSink.html
[wsstart]: https://docs.rs/ntex/latest/ntex/web/ws/fn.start.html
[service]: https://docs.rs/ntex/latest/ntex/trait.Service.html
[examples]: https://github.com/ntex-rs/examples/tree/master/websocket
[chat]: https://github.com/ntex-rs/examples/tree/master/websocket-chat
