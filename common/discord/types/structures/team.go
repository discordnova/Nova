package structures

// Team represents a Discord Team
type Team struct {
	// a hash of the image of the team's icon
	Icon string `json:"icon,omitempty"`
	// the unique id of the team
	ID string `json:"id"`
	// 	the members of the team
	Members []TeamMember `json:"members"`
	// the user id of the current team owner
	OwnerUserID string `json:"owner_user_id"`
}
