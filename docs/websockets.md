---
title: Websockets
---

import CodeBlock from "@site/src/components/code_block.js";

# Websockets

Ntex supports WebSockets out of the box. It is possible to convert a request's `Payload` to a stream of [_ws::Message_][message] with a [_web::Payload_][payload] and then use stream combinators to handle actual messages, but it is simpler to handle websocket communications with an http service.

The following is an example of a simple websocket echo server:


<CodeBlock example="websockets" file="main.rs" section="websockets" />

> A simple websocket echo server example is available in the [examples directory][examples].

> An example chat server with the ability to chat over a websocket or TCP connection is available in [websocket-chat directory][chat]

[message]: https://docs.rs/ntex/latest/ntex/ws/enum.Message.html
[payload]: https://docs.rs/ntex/latest/ntex/web/types/struct.Payload.html
[examples]: https://github.com/ntex-rs/examples/tree/master/websocket
[chat]: https://github.com/ntex-rs/examples/tree/master/websocket-chat
