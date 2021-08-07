package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildMemberAddPayload is sent when a new user joins a guild.
// The inner payload is a guild member object with an extra guild_id key
// If using Gateway Intents, the GUILD_MEMBERS intent will be required to receive this event.
type GatewayEventGuildMemberAddPayload struct {
	structures.GuildMember
	// id of the guild
	GuildID string `json:"guild_id"`
}
