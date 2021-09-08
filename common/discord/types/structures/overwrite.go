package structures

// Overwrite is a representation of Overwrite in Discord permissions system
type Overwrite struct {
	// role or user id
	ID string `json:"id"`
	// either 0 (role) or 1 (member)
	Type int `json:"type"`
	// permission bit set
	Allow string `json:"allow"`
	// permission bit set
	Deny string `json:"deny"`
}
