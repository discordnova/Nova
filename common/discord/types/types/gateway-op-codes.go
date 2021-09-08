package types

// GatewayOpCodes are code tagged by the Discord Gateway to type payloads
type GatewayOpCodes int

// Valid OpCode values
const (
	// GatewayOpCodeDispatch is when an event was dispatched.
	GatewayOpCodeDispatch GatewayOpCodes = 0
	// GatewayOpCodeHeartbeat is fired periodically by the client
	// to keep the connection alive.
	GatewayOpCodeHeartbeat = 1
	// GatewayOpCodeIdentify starts a new session during the initial handshake.
	GatewayOpCodeIdentify = 2
	// GatewayOpCodePresenceUpdate updates the client's presence.
	GatewayOpCodePresenceUpdate = 3
	// GatewayOpCodeVoiceStateUpdate is used to join/leave or
	// move between voice channels.
	GatewayOpCodeVoiceStateUpdate = 4

	// Yes, there is no 5

	// GatewayOpCodeResume resumes a previous session that was disconnected.
	GatewayOpCodeResume = 6
	// GatewayOpCodeReconnect announces that you should attempt to reconnect
	// and resume immediately.
	GatewayOpCodeReconnect = 7
	// GatewayOpCodeRequestGuildMembers requests information about offline guild
	// members in a large guild.
	GatewayOpCodeRequestGuildMembers = 8
	// GatewayOpCodeInvalidSession announces that the session has been invalidated and
	// you should reconnect and identify/resume accordingly.
	GatewayOpCodeInvalidSession = 9
	// GatewayOpCodeHello is sent immediately after connecting and contains
	// the heartbeat_interval to use.
	GatewayOpCodeHello = 10
	// GatewayOpCodeHeartbeatACK is sent in response to receiving a heartbeat
	// to acknowledge that it has been received.
	GatewayOpCodeHeartbeatACK = 11
)
