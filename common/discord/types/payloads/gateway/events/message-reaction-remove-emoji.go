package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventMessageReactionRemoveEmojiPayload is sent when a bot removes all instances of a given emoji from the
// reactions of a message.
type GatewayEventMessageReactionRemoveEmojiPayload struct {
	// the id of the channel
	ChannelID string `json:"channel_id"`
	// the id of the guild
	GuildID string `json:"guild_id,omitempty"`
	// the id of the message
	MessageID string `json:"message_id"`
	// the emoji that was removed
	Emoji structures.Emoji `json:"emoji"`
}
