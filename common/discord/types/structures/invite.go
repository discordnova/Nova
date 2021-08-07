package structures

import "github.com/discordnova/nova/common/discord/types/types"

// Invite is the representation of a Discord Invite
type Invite struct {
	// the invite code (unique ID)
	Code string `json:"code"`
	// the guild this invite is for
	Guild Guild `json:"guild,omitempty"`
	// the channel this invite is for
	Channel Channel `json:"channel"`
	// the user who created the invite
	Inviter User `json:"inviter,omitempty"`
	// the target user for this invite
	TargetUser User `json:"target_user,omitempty"`
	// the type of user target for this invite
	TargetUserType types.TargetUserTypes `json:"target_user_type,omitempty"`
	// approximate count of online members (only present when target_user is set)
	ApproximatePresenceCount int `json:"approximate_presence_count,omitempty"`
	// approximate count of total members
	ApproximateMemberCount int `json:"approximate_member_count"`
}
