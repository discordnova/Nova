package types

// GatewayCloseEventCodes is a code explaining the reason
// the gateway connection closed
type GatewayCloseEventCodes int

// GatewayCloseEventCodes values
const (
	// GatewayCloseEventCodeUnknownError is corresponding to a
	// Discord internal error
	GatewayCloseEventCodeUnknownError GatewayCloseEventCodes = 4000
	// GatewayCloseEventCodeUnknownOpCode is used when an
	// invalid payload for a  payload or opcode was sent
	GatewayCloseEventCodeUnknownOpCode = 4001
	// GatewayCloseEventCodeDecodeError is used when an invalid payload was sent
	GatewayCloseEventCodeDecodeError = 4002
	// GatewayCloseEventCodeNotAuthenticated is used when a payload was
	// sent prior to the Identify payload
	GatewayCloseEventCodeNotAuthenticated = 4003
	// GatewayCloseEventCodeAuthenticationFailed is used when the account token
	// in the Identify payload is incorrect
	GatewayCloseEventCodeAuthenticationFailed = 4004
	// GatewayCloseEventCodeAlreadyAuthenticated is used when more than one
	// Identify payload was sent
	GatewayCloseEventCodeAlreadyAuthenticated = 4005

	/* There is no 4006... */

	// GatewayCloseEventCodeInvalidSeq is used when the sequence of a
	// resuming payload is invalid. Need tro reconnect and start a new session
	GatewayCloseEventCodeInvalidSeq = 4007
	// GatewayCloseEventCodeRateLimited is used when a rate limit applied.
	GatewayCloseEventCodeRateLimited = 4008
	// GatewayCloseEventCodeSessionTimedOut is used when a session expired.
	// Need to reconnect and start a new one
	GatewayCloseEventCodeSessionTimedOut = 4009
	// GatewayCloseEventCodeInvalidShard is used when an invalid shard was sent
	// in the Identify payload
	GatewayCloseEventCodeInvalidShard = 4010
	// GatewayCloseEventCodeShardingRequired is used when a session handle too
	// many guild. The connection needs more shards
	GatewayCloseEventCodeShardingRequired = 4011
	// GatewayCloseEventCodeInvalidAPIVersion is used when an invalid version
	// of the gateway was sent
	GatewayCloseEventCodeInvalidAPIVersion = 4012
	// GatewayCloseEventCodeInvalidIntents is used when invalid intents were
	// provided.
	GatewayCloseEventCodeInvalidIntents = 4013
	// GatewayCloseEventCodeDisallowedIntents is used when a provided intent is
	// not enabled.
	GatewayCloseEventCodeDisallowedIntents = 4014
)
