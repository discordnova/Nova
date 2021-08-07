package events

// GatewayEventWebhookUpdatePayload is sent when a guild channel's webhook is created, updated, or deleted.
type GatewayEventWebhookUpdatePayload struct {
	// id of the guild
	GuildID string `json:"guild_id"`
	// id of the channel
	ChannelID string `json:"channel_id"`
}
