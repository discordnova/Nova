package events

import (
	"github.com/discordnova/nova/common/discord/types/structures"
	"github.com/discordnova/nova/common/discord/types/types"
)

// GatewayEventInviteCreatePayload is sent when a new invite to a channel is created.
type GatewayEventInviteCreatePayload struct {
	// the channel the invite is for
	ChannelID string `json:"channel_id"`
	// the unique invite code
	Code string `json:"code"`
	// the time at which the invite was created
	CreatedAt string `json:"created_at"`
	// the guild of the invite
	GuildID string `json:"guild_id,omitempty"`
	// the user that created the invite
	Inviter structures.User `json:"inviter,omitempty"`
	// how long the invite is valid for (in seconds)
	MaxAge int `json:"max_age"`
	// the maximum number of times the invite can be used
	MaxUses int `json:"max_uses"`
	// the target user for this invite
	TargetUser structures.User `json:"target_user,omitempty"`
	// the type of user target for this invite
	TargetUserType types.TargetUserTypes `json:"target_user_type"`
	// whether or not the invite is temporary (invited users will be kicked on disconnect unless they're assigned a role)
	Temporary bool `json:"temporary"`
	// how many times the invite has been used (always will be 0)
	Uses int `json:"uses"`
}
