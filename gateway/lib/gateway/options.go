package gateway

import (
	"github.com/discordnova/nova/common/discord/types/types"
	"github.com/discordnova/nova/common/gateway"
)

// GatewayConnectorOptionsResume represents the options for reconnecting the gateway.
type GatewayConnectorOptionsResume struct {
	Session string `json:"session_id"` // The session id of the older session we want to resume.
	Index   int64  `json:"index"`      // The index of the last packet recevied by the older session.
}

// GatewayConnectorOptionsSharding represents the options for sharding the gateway.
type GatewayConnectorOptionsSharding struct {
	TotalShards  int `json:"total_shards"`  // The total amount of shards
	CurrentShard int `json:"current_shard"` // The shard we want to connect to.
}

// GatewayConnectorOptions is the options given to the GatewayConnector when creating it.
type GatewayConnectorOptions struct {
	Token         *string                       // The token of the bot
	SelfShard     *int                          // The shard of the current connector
	TotalShard    *int                          // The total count of shards
	Intend        types.GatewayIntents          // The bitflags for the indents.
	GuildSubs     *bool                         // Should the guild_subscriptions be enabled
	ResumeSession GatewayConnectorOptionsResume // Is specified, the gateway will try to resume a connection.
	Compressor    gateway.Compression           // The compressor given to the gateway that determine the connection method and compression used.
	Transporter   gateway.Transporter           // The interface where we send the data.
	Restart       *bool                         // Should the gateway restart upon failure.

	OnSessionStateUpdated  func(state GatewayConnectorOptionsResume) error // When the session state is called, we call this function
	SessionUpdateFrequency *int
}
