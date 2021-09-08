package structures

// MessageReference is the reference object of a message
type MessageReference struct {
	// id of the originating message
	MessageID string `json:"message_id,omitempty"`
	// id of the originating message's channel
	// channel_id is optional when creating a reply, but will always be present when receiving an event/response
	// that includes this data model.
	ChannelID string `json:"channel_id,omitempty"`
	// id of the originating message's guild
	GuildID string `json:"guild_id,omitempty"`
	// when sending, whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply)
	// message, default true
	FailIfNotExists bool `json:"fail_if_not_exists,omitempty"`
}
