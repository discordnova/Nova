package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildRoleUpdatePayload is sent when a guild role is updated.
type GatewayEventGuildRoleUpdatePayload struct {
	// the id of the guild
	GuildID string `json:"guild_id"`
	// the role updated
	Role structures.Role `json:"role"`
}
