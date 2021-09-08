package structures

// ActivityParty is a representation of a party sent in an Activity
type ActivityParty struct {
	// the id of the party
	ID string `json:"id,omitempty"`
	// used to show the party's current and maximum size
	// This is an array of two integers (current_size, max_size)
	Size []int `json:"size,omitempty"`
}
