package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildBanRemovePayload is sent when a user is unbanned from a guild.
type GatewayEventGuildBanRemovePayload struct {
	// id of the guild
	GuildID string `json:"guild_id"`
	// the unbanned user
	User structures.User `json:"user"`
}
