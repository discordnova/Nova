package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildRoleCreatePayload is sent when a guild role is created.
type GatewayEventGuildRoleCreatePayload struct {
	// the id of the guild
	GuildID string `json:"guild_id"`
	// the role created
	Role structures.Role `json:"role"`
}
