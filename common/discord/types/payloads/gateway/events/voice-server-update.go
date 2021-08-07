package events

// GatewayEventVoiceServerUpdate is sent when a guild's voice server is updated.
type GatewayEventVoiceServerUpdate struct {
	// voice connection token
	Token string `json:"token"`
	// the guild this voice server update is for
	GuildID string `json:"guild_id"`
	// the voice server host
	Endpoint string `json:"endpoint"`
}
