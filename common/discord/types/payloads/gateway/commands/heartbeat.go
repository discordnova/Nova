package commands

// GatewayCommandHeartbeat is used  to maintain an active gateway connection.
// Must be sent every heartbeat_interval milliseconds after the Opcode 10 Hello payload is received.
// The inner d key is the last sequence number—s—received by the client. If you have not yet received one, send null.
type GatewayCommandHeartbeat int
