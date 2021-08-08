# Ratelimiter

- [Ratelimiter](#ratelimiter)
  - [Explanation](#explanation)
  - [Binary](#binary)
  - [Library](#library)
  - [Algorithm](#algorithm)
  - [Additional information](#additional-information)

## Explanation

Ratelimiter is a Rust implementation of a rate limiting service described by [`proto/nova_ratelimit_v1.proto`](proto/nova_ratelimit_v1.proto).
It manages the rate limits of Nova, by exposing a gRPC Server.

The service is composed of two parts:

- a [binary](#binary) which implements the algorithm logic,
- a [library](#library) which is a static library.

---

## Binary

The Ratelimiter service binary is a simple Rust program which expose a gRPC Service defined by [`proto/nova_ratelimit_v1.proto`](proto/nova_ratelimit_v1.proto) on `[::0]:50051` (quick representation for `IPv4/0.0.0.0` or `IPv6/::`, on port `50051`) using [tonic](https://github.com/hyperium/tonic).

The binary also communicates with a Redis Node to store buckets by [the algorithm](#algorithm).
It supports both `redis` and `rediss` protocol and, we are working on a Redis Cluster support.

## Library

> TODO: make good documentation while making the lib
Static lib + [FFI interface](https://en.wikipedia.org/wiki/Foreign_function_interface) for nova-lite

---

## Algorithm

The algorithm used by the service is pretty simple.

On the reception of a `nova.rate limit.v1.RatelimitService/GetRatelimitStatus` call, the service computes a `SHA256` hash using the content of the call request *(see code below)*.

> From [`proto/nova_ratelimit_v1.proto`](proto/nova_ratelimit_v1.proto)

```protobuf
// Requests the ratelimit status of a route request, it also takes the 
// identifiers of the request in question.
message RatelimitRequest {
    string          routeName       = 1;
    repeated string identifiers  = 2;
}
```

It then uses the calculated hash to check if a bucket already exists in our Redis store with this hash.

If not, the service locks the bucket and disallows new buckets creation for some time. It also responds with a `Status.STATUS_OK`, and we set `update_asked` to `true` to ask for bucket information to create one separately.

Otherwise, we need to check if enough are remaining allowed requests *(`remaining` > 1)* for the bucket. If not, the Ratelimiter responds with `Status.STATUS_RATELIMITED`.

If there is enough, we decrement `remaining` by one and check if enough is remaining allowed request *(`remaining` > 1)* for the global rate limit. If not, the request ends with a `Status.STATUS_GLOBAL_RATELIMITED` status.

If the client isn't rate-limited, we decrement again `remaining` by one and respond with `Status.STATUS_OK` and `update_asked` to `false`.

---

## Additional information

More information at [the main README.md file](../README.md).
