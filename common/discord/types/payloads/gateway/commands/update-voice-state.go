package commands

// GatewayCommandUpdateVoiceStatePayload is sent when a client wants to join, move, or disconnect from a voice channel.
type GatewayCommandUpdateVoiceStatePayload struct {
	// id of the guild
	GuildID string `json:"guild_id"`
	// id of the voice channel client wants to join (null if disconnecting)
	ChannelID string `json:"channel_id,omitempty"`
	// is the client muted
	SelfMute bool `json:"self_mute"`
	// is the client deafened
	SelfDeaf bool `json:"self_deaf"`
}
