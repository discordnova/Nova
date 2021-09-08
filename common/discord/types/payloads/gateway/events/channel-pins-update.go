package events

// GatewayEventChannelPinsUpdatePayload is sent when a message is pinned or unpinned in a text channel.
// This is not sent when a pinned message is deleted.
type GatewayEventChannelPinsUpdatePayload struct {
	// the id of the guild
	GuildID string `json:"guild_id,omitempty"`
	// the id of the channel
	ChannelID string `json:"channel_id"`
	// the time at which the most recent pinned message was pinned
	LastPinTimestamp string `json:"last_pin_timestamp,omitempty"`
}
