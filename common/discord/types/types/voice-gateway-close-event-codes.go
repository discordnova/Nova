package types

// VoiceGatewayCloseEventCodes is a code explaining the close of a connection
// to the Voice gateway
type VoiceGatewayCloseEventCodes int

// Valid codes
const (
	// VoiceGatewayCloseEventCodeUnknownOpcode is used after sending an
	// invalid Opcode
	VoiceGatewayCloseEventCodeUnknownOpcode VoiceGatewayCloseEventCodes = 4001
	// VoiceGatewayCloseEventCodeFailedToDecodePayload is used after sending an
	// invalid payload
	VoiceGatewayCloseEventCodeFailedToDecodePayload = 4002
	// VoiceGatewayCloseEventCodeNotAuthenticated is used after sending a payload
	// prior to the Identifying one.
	VoiceGatewayCloseEventCodeNotAuthenticated = 4003
	// VoiceGatewayCloseEventCodeAuthenticationFailed is used after sending an
	// invalid Identify payload
	VoiceGatewayCloseEventCodeAuthenticationFailed = 4004
	// VoiceGatewayCloseEventCodeAlreadyAuthenticated is used when sending more
	// than ono Identify payload
	VoiceGatewayCloseEventCodeAlreadyAuthenticated = 4005
	// VoiceGatewayCloseEventCodeSessionNoLongerValid is used when a session is
	// no longer valid
	VoiceGatewayCloseEventCodeSessionNoLongerValid = 4006

	/* there is no 4007 and 4008 */

	// VoiceGatewayCloseEventCodeSessionTimeout is used when a session timed out
	VoiceGatewayCloseEventCodeSessionTimeout = 4009

	/* no 4010 :angry_face: */

	// VoiceGatewayCloseEventCodeServerNotFound is used after asking for an
	// invalid voice server
	VoiceGatewayCloseEventCodeServerNotFound = 4011
	// VoiceGatewayCloseEventCodeUnknownProtocol is used after sending an
	// unrecognized protocol
	VoiceGatewayCloseEventCodeUnknownProtocol = 4012

	/* There is no 1043!!!! */

	// VoiceGatewayCloseEventCodeDisconnected is used when the client has
	// been disconnected and should not reconnect (deleted channel, kicked...)
	VoiceGatewayCloseEventCodeDisconnected = 4014
	// VoiceGatewayCloseEventCodeVoiceServerCrashed is used after a voice
	// server crashed. Need to resume the connection...
	VoiceGatewayCloseEventCodeVoiceServerCrashed = 4015
	// VoiceGatewayCloseEventCodeUnknownEncryptionMode is used after sending a
	// payload with an unrecognized encryption
	VoiceGatewayCloseEventCodeUnknownEncryptionMode = 4016
)
