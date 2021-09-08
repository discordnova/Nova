package structures

// WelcomeScreenChannel is a representation of a channel in a welcome screen
type WelcomeScreenChannel struct {
	// the channel's id
	ChannelID string `json:"channel_id"`
	// the description shown for the channel
	Description string `json:"description"`
	// the emoji id, if the emoji is custom
	EmojiID string `json:"emoji_id,omitempty"`
	// the emoji name if custom, the unicode character if standard, or null if no emoji is set
	EmojiName string `json:"emoji_name,omitempty"`
}
