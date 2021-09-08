package events

// GatewayEventInviteDeletePayload is sent when an invite is deleted.
type GatewayEventInviteDeletePayload struct {
	// the channel of the invite
	ChannelID string `json:"channel_id"`
	// the guild of the invite
	GuildID string `json:"guild_id"`
	// the unique invite code
	Code string `json:"code"`
}
