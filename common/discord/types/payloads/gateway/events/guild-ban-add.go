package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildBanAddPayload is sent when a user is banned from a guild.
type GatewayEventGuildBanAddPayload struct {
	// id of the guild
	GuildID string `json:"guild_id"`
	// the banned user
	User structures.User `json:"user"`
}
