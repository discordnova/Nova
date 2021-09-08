package commands

import (
	"github.com/discordnova/nova/common/discord/types/structures"
	"github.com/discordnova/nova/common/discord/types/types"
)

// GatewayCommandIdentifyPayload is used to trigger the initial handshake with the gateway.
type GatewayCommandIdentifyPayload struct {
	// authentication token
	Token string `json:"token"`
	// connection properties
	Properties structures.IdentifyConnectionProperties `json:"properties"`
	// whether this connection supports compression of packets
	// Default to false
	Compress bool `json:"compress,omitempty"`
	// value between 50 and 250, total number of members where the gateway will stop sending offline members in the
	// guild member list
	// Default to 50
	LargeThreshold int `json:"large_threshold,omitempty"`
	// used for Guild Sharding. array of two integers (shard_id, num_shards)
	Shard []int `json:"shard,omitempty"`
	// presence structure for initial presence information
	Presence GatewayCommandUpdateStatusPayload `json:"presence,omitempty"`
	// enables dispatching of guild subscription events (presence and typing events)
	// Default to true
	GuildSubscriptions bool `json:"guild_subscriptions,omitempty"`
	// the Gateway Intents you wish to receive
	Intents types.GatewayIntents `json:"intents"`
}
