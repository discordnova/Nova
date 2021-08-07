package structures

// Reaction represents a message reaction
type Reaction struct {
	// 	times this emoji has been used to react
	Count int `json:"count"`
	// whether the current user reacted using this emoji
	Me bool `json:"me"`
	// emoji information
	Emoji Emoji `json:"emoji"`
}
