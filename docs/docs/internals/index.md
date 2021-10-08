---
sidebar_position: 1
---

# Nova Internals

## Definitions

### Cache

A cache is an instance of the `cache` program, it managed everything that is related to data-management
of the discord events, webhooks and more.

### Gateway
*todo*
### Webhook
*todo*
### 

## Communication

```
    ┌───────────────┐              ┌───────────────────┐
    │    Gateway    │   event(s)   │                   │                   ┌───────────────────────┐
┌──►│   (gateway)   ├─────────────►│                   ├──────────────────►│                       │
│   └───────────────┘ (nats queue) │   Cache instance  │  (redis channel)  │     Caching server    │
│                                  │      (cache)      │                   │        (redis)        │
│   ┌───────────────┐   event(s)   │                   │◄──────────────────┤                       │
│   │    Webhook    ├─────────────►│                   │                   └───────────────────────┘
│   │   (webhook)   │ (nats queue) └─────────────────┬─┘
│   └───────────────┘                  ▲             │
│                       cache requests │             │   event(s)
│                          (grpc)      │             │ (nats queue)
│                                      │             ▼
│                                  ┌───┴───────────────┐
└──────────────────────────────────┤   User programs   │
         gateway commands          │   (client libs)   │
              (grpc)               └───────────────────┘
```

> We try to use protocol buffers everywhere even inside the event broker (nats),
> this ensures we have the same date representation across all the programs written
> in different languages

> The only dependency of the nova architecture is a reliable nats & redis cluster for now

#### Protocols

All the protocols a detailed in the [protocols folder](protocols)

#### 