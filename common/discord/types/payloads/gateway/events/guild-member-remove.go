package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildMemberRemovePayload is sent when a user is removed from a guild (leave/kick/ban).
// If using Gateway Intents, the GUILD_MEMBERS intent will be required to receive this event.
type GatewayEventGuildMemberRemovePayload struct {
	// the id of the guild
	GuildID string `json:"guild_id"`
	// the user who was removed
	User structures.User `json:"user"`
}
