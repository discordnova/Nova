package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildEmojisUpdatePayload is sent when when a guild's emojis have been updated.
type GatewayEventGuildEmojisUpdatePayload struct {
	// id of the guild
	GuildID string `json:"guild_id"`
	// array of emojis
	Emojis []structures.Emoji `json:"emojis"`
}
