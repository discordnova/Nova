package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventReadyPayload is the ready event is dispatched when a client has
// completed the initial handshake with the gateway (for new sessions).
// The ready event can be the largest and most complex event the gateway
// will send, as it contains all the state required for a client
// to begin interacting with the rest of the platform.
type GatewayEventReadyPayload struct {
	// V is the gateway version
	V int `json:"v"`
	// User is information about the user including email
	User structures.User `json:"user"`
	// PrivateChannels is an empty array
	PrivateChannels []string `json:"private_channels"`
	// Guilds are the guilds the user is in
	// Note: they are only sent as unavailable guilds
	Guilds []structures.Guild `json:"guilds"`
	// SessionID is used for resuming connections
	SessionID string `json:"session_id"`
	// Shard is the shard information associated with the session, if sent
	// when identifying. This is an array of two integers (shard_id, num_shards)
	Shard []int `json:"shard,omitempty"`
	// contains id and flags
	Application structures.Application `json:"application"`
}
