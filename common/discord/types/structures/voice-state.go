package structures

// VoiceState is a representation of a user's voice connection status
type VoiceState struct {
	// the guild id this voice state is for
	GuildID string `json:"guild_id,omitempty"`
	// the channel id this user is connected to
	ChannelID string `json:"channel_id,omitempty"`
	// the user id this voice state is for
	UserID string `json:"user_id,omitempty"`
	// the guild member this voice state is for
	Member GuildMember `json:"member,omitempty"`
	// the session id for this voice state
	SessionID string `json:"session_id"`
	// whether this user is deafened by the server
	Deaf bool `json:"deaf"`
	// whether this user is muted by the server
	Mute bool `json:"mute"`
	// whether this user is locally deafened
	SelfDeaf bool `json:"self_deaf"`
	// whether this user is locally muted
	SelfMute bool `json:"self_mute"`
	// whether this user is streaming using "Go Live"
	SelfStream bool `json:"self_stream,omitempty"`
	// whether this user's camera is enabled
	SelfVideo bool `json:"self_video"`
	// whether this user is muted by the current user
	Suppress bool `json:"suppress"`
}
