package structures

// ActivityEmoji is the representation of an Emoji object sent for Activities
type ActivityEmoji struct {
	// the name of the emoji
	Name string `json:"name"`
	// the id of the emoji
	ID string `json:"id,omitempty"`
	// whether this emoji is animated
	Animated bool `json:"animated,omitempty"`
}
