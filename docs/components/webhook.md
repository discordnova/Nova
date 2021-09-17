# Webhook

> TD;TR The webhook component is an implementation of the discord interaction webhooks; You can either use the Gateway or the Webhooks. The webhooks __require__ an external https endpoint to work.

The webhook source code is located in the [webhook](../../webhook) folder and is implemented in Rust. It's a simple http web server which implements the webhook signature verification and deserialization. Like the gateway, the messages are redirected using the [relaying system](../common#relaying_trait).

The signature verification is done using libsodium via the libsodium-sys trait.
Subsequently, it uses code marked as "unsafe" in rust. It's built into the binary statically. Any route can be used to receive webhook messages.