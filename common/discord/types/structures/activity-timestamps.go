package structures

// ActivityTimestamps are the timestamps of an activity
type ActivityTimestamps struct {
	// unix time (in milliseconds) of when the activity started
	Start int `json:"start,omitempty"`
	// unix time (in milliseconds) of when the activity ends
	End int `json:"end,omitempty"`
}
