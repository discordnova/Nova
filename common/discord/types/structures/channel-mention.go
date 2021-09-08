package structures

import "github.com/discordnova/nova/common/discord/types/types"

// ChannelMention is a representation of a Channel mention
type ChannelMention struct {
	// id of the channel
	ID string `json:"id"`
	// id of the guild containing the channel
	GuildID string `json:"guild_id"`
	// the type of channel
	Type types.ChannelTypes `json:"type"`
	// the name of the channel
	Name string `json:"name"`
}
