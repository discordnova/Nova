package events

// GatewayEventInvalidSessionPayload is sent to indicate one of at least three different situations:the gateway could
// not initialize a session after receiving an Opcode 2 Identify the gateway could not resume un anterior session
// after receiving an Opcode 6 Resume the gateway has invalidated an active session and is requesting client
// actionThe inner d key is a boolean that indicates whether the session may be resumable.
// See Connecting and Resuming for more information.
type GatewayEventInvalidSessionPayload bool
