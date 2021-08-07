package structures

import "github.com/discordnova/nova/common/discord/types/types"

// TeamMember represents a member of a Discord Team
type TeamMember struct {
	// the user's membership state on the team
	MembershipState types.MembershipStates `json:"membership_state"`
	// will always be ["*"]
	Permissions []string `json:"permissions"`
	// the id of the parent team of which they are a member
	TeamID string `json:"team_id"`
	// the avatar, discriminator, id, and username of the user
	User User `json:"user"`
}
