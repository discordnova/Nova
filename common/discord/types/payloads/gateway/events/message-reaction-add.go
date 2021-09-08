package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventMessageReactionAdd is sent when a user adds a reaction to a message.
type GatewayEventMessageReactionAdd struct {
	// the id of the user
	UserID string `json:"user_id"`
	// the id of the channel
	ChannelID string `json:"channel_id"`
	// the id of the message
	MessageID string `json:"message_id"`
	// the id of the guild
	GuildID string `json:"guild_id,omitempty"`
	// the member who reacted if this happened in a guild
	Member structures.GuildMember `json:"member"`
	// the emoji used to react
	Emoji structures.Emoji `json:"emoji"`
}
