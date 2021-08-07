package events

// GatewayEventMessageBulkDeletePayload is sent when multiple messages are deleted at once.
type GatewayEventMessageBulkDeletePayload struct {
	// 	the ids of the messages
	IDs []string `json:"ids"`
	// 	the id of the channel
	ChannelID string `json:"channel_id"`
	// the id of the guild
	GuildID string `json:"guild_id,omitempty"`
}
