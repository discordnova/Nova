package structures

// Emoji is a representation of a Discord Emoji
type Emoji struct {
	// ID is the emoji id and can be null for default Discord emojis
	ID string `json:"id,omitempty"`
	// Name is the emoji name and can only be nil in Reaction Emoji objects
	Name string `json:"name,omitempty"`
	// 	roles this emoji is whitelisted to
	Roles string `json:"roles,omitempty"`
	// user that created this emoji
	User User `json:"user,omitempty"`
	// whether this emoji must be wrapped in colons
	RequireColons bool `json:"require_colons,omitempty"`
	// whether this emoji is managed
	Managed bool `json:"managed,omitempty"`
	// whether this emoji is animated
	Animated bool `json:"animated,omitempty"`
	// whether this emoji can be used, may be false due to loss of Server Boosts
	Available bool `json:"available,omitempty"`
}
