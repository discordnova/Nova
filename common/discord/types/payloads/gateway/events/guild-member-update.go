package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildMemberUpdatePayload is sent when a guild member is updated.
// This will also fire when the user object of a guild member changes.
// If using Gateway Intents, the GUILD_MEMBERS intent will be required to receive this event.
type GatewayEventGuildMemberUpdatePayload struct {
	// 	the id of the guild
	GuildID string `json:"guild_id"`
	// user role ids
	Roles []string `json:"roles"`
	// the user
	User structures.User `json:"user"`
	// nickname of the user in the guild
	Nick string `json:"nick,omitempty"`
	// when the user joined the guild
	JoinedAt string `json:"joined_at"`
	// when the user starting boosting the guild
	PremiumSince string `json:"premium_since,omitempty"`
	// whether the user has not yet passed the guild's Membership Screening requirements
	Pending bool `json:"pending"`
}
