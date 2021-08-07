package events

// GatewayEventMessageDeletePayload is sent when a message is deleted.
type GatewayEventMessageDeletePayload struct {
	// the id of the message
	ID string `json:"id"`
	// 	the id of the channel
	ChannelID string `json:"channel_id"`
	// the id of the guild
	GuildID string `json:"guild_id"`
}
