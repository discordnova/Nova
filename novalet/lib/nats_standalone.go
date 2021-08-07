package lib

import (
	stand "github.com/nats-io/nats-streaming-server/server"
)

type NatsStandalone struct {
	streamingServer *stand.StanServer
}

func NewNatsStandalone() *NatsStandalone {
	server, err := stand.RunServer("standalone_server")
	if err != nil {
		panic("failed to start the server")
	}
	return &NatsStandalone{
		streamingServer: server,
	}
}
