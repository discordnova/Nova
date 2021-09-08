package structures

// ActivitySecrets is a representation of the secrets object sent in an Activity
type ActivitySecrets struct {
	// the secret for joining a party
	Join string `json:"join,omitempty"`
	// the secret for spectating a game
	Spectate string `json:"spectate,omitempty"`
	// the secret for a specific instanced match
	Match string `json:"match,omitempty"`
}
