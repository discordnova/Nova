---
title: Gateway
position: 1
---

The gateway if the component responsible for the real-time events
using the Discord Gateway API (websocket), the component is written
in rust using [`tokio-tungstenite`](https://github.com/snapview/tokio-tungstenite) and his job is to simply maintain
a gateway connection (heartbeat, reconnect, etc...) and send events
directly to the cache using nats.

The project is located in the `gateway` folder at the root
of the GitHub repo and is published using a statically linked
library (except libc) and docker images.

## Structure

Internally, the gateway is divided in multiple rust modules

### Connection (src/connection)

This module implements a [`futures_core::stream::Stream` trait](https://docs.rs/futures-core/0.3.17/futures_core/stream/trait.Stream.html)
and [`futures::sink::Sink` trait](https://docs.rs/futures/0.3.17/futures/sink/trait.Sink.html) that streams deserialized discord
packets to the stream. It does not implement any reconnect mechanism and returns
an error when the connection encounters a deserialization error or i/o error.
If the connection is closed, the stream simply closes.

You can send any `BaseMessage` struct through the connection and receive
any `BaseMessage` struct.


### Payloads (src/payload)

This module implements the deserialization of discord packets
