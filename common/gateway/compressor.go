package gateway

import "github.com/discordnova/nova/common/discord/types/payloads/gateway"

// GatewayConnectionOptions is the options given to gateway when the connector is passed to it.
type GatewayConnectionOptions struct {
	Encoding             string
	TransportCompression string
}

// Compression is the interface that needs to be implemented by a generic compressor.
type Compression interface {
	GetConnectionOptions() GatewayConnectionOptions
	DecodeMessage(data []byte) (*gateway.Payload, error)
	Reset() error
}
