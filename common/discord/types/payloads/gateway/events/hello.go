package events

// GatewayEventHelloPayload is sent on connection to the websocket.
// Defines the heartbeat interval that the client should heartbeat to.
type GatewayEventHelloPayload struct {
	// HeartbeatInterval is the interval (in milliseconds)
	// the client should heartbeat with
	HeartbeatInterval int `json:"heartbeat_interval"`
}
