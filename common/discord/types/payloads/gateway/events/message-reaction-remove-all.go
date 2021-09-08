package events

// GatewayEventMessageReactionRemoveAll is sent when a user explicitly removes all reactions from a message.
type GatewayEventMessageReactionRemoveAll struct {
	// the id of the channel
	ChannelID string `json:"channel_id"`
	// the id of the message
	MessageID string `json:"message_id"`
	// the id of the guild
	GuildID string `json:"guild_id,omitempty"`
}
