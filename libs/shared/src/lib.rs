#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
)]

pub mod config;
pub mod nats;
pub mod payloads;
pub mod redis;
pub mod opentelemetry;