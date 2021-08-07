package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventMessageReactionRemove is sent when a user removes a reaction from a message.
type GatewayEventMessageReactionRemove struct {
	// the id of the user
	UserID string `json:"user_id"`
	// the id of the channel
	ChannelID string `json:"channel_id"`
	// the id of the message
	MessageID string `json:"message_id"`
	// the id of the guild
	GuildID string `json:"guild_id,omitempty"`
	// the emoji used to react
	Emoji structures.Emoji `json:"emoji"`
}
