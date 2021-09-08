package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventTypingStartPayload is sent when a user starts typing in a channel.
type GatewayEventTypingStartPayload struct {
	// id of the channel
	ChannelID string `json:"channel_id"`
	// id of the guild
	GuildID string `json:"guild_id,omitempty"`
	// id of the user
	UserID string `json:"user_id"`
	// unix time (in seconds) of when the user started typing
	Timestamp int `json:"timestamp"`
	// the member who started typing if this happened in a guild
	Member structures.GuildMember `json:"member,omitempty"`
}
