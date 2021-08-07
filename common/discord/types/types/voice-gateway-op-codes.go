package types

// VoiceGatewayOpCodes are code tagged by the Discord Gateway to type payloads
type VoiceGatewayOpCodes int

// Valid OpCode values
const (
	// VoiceGatewayOpCodeIdentify is used to begin a voice websocket connection
	VoiceGatewayOpCodeIdentify VoiceGatewayOpCodes = 0
	// VoiceGatewayOpCodeSelectProtocol is used to select the voice protocol
	VoiceGatewayOpCodeSelectProtocol = 1
	// VoiceGatewayOpCodeReady is used to complete the websocket handshake
	VoiceGatewayOpCodeReady = 2
	// VoiceGatewayOpCodeHeartbeat is used to keep the connection alive
	VoiceGatewayOpCodeHeartbeat = 3
	// VoiceGatewayOpCodeSessionDescription is used to describe the session
	VoiceGatewayOpCodeSessionDescription = 4
	// VoiceGatewayOpCodeSpeaking is used to indicate which users are speaking
	VoiceGatewayOpCodeSpeaking = 5
	// VoiceGatewayOpCodeHeartbeatACK is used to acknowledge a
	// received heartbeat
	VoiceGatewayOpCodeHeartbeatACK = 6
	// VoiceGatewayOpCodeResume is used to resume a connection
	VoiceGatewayOpCodeResume = 7
	// VoiceGatewayOpCodeHello is used to give the time to wait between sending
	// heartbeats in milliseconds
	VoiceGatewayOpCodeHello = 8
	// VoiceGatewayOpCodeResumed is used to acknowledge a successful
	// session resume
	VoiceGatewayOpCodeResumed = 9

	/* there is no 10, 11 and 12 :/ */

	// VoiceGatewayOpCodeClientDisconnect is used to announce the client
	// has disconnected from the voice channel
	VoiceGatewayOpCodeClientDisconnect = 13
)
