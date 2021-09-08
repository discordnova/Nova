package structures

import "github.com/discordnova/nova/common/discord/types/types"

// Channel represents a guild or DM channel
type Channel struct {
	// the id of this channel
	ID string `json:"id"`
	// the type of channel
	Type types.ChannelTypes `json:"type"`
	// the id of the guild
	GuildID string `json:"guild_id,omitempty"`
	// sorting position of the channel
	Position int `json:"position,omitempty"`
	// explicit permission overwrites for members and roles
	PermissionOverwrites []Overwrite `json:"permission_overwrites,omitempty"`
	// the name of the channel (2-100 characters)
	Name string `json:"name,omitempty"`
	// the channel topic (0-1024 characters)
	Topic string `json:"topic,omitempty"`
	// whether the channel is nsfw
	NSFW bool `json:"nsfw,omitempty"`
	// the id of the last message sent in this channel (may not point to an existing or valid message)
	LastMessageID string `json:"last_message_id,omitempty"`
	// the bitrate (in bits) of the voice channel
	Bitrate int `json:"bitrate,omitempty"`
	// the user limit of the voice channel
	UserLimit int `json:"user_limit,omitempty"`
	// amount of seconds a user has to wait before sending another message (0-21600);
	// bots, as well as users with the permission manage_messages or manage_channel, are unaffected
	RateLimitPerUser int `json:"rate_limit_per_user,omitempty"`
	// the recipients of the DM
	Recipients []User `json:"recipients,omitempty"`
	// icon hash
	Icon string `json:"icon,omitempty"`
	// id of the DM creator
	OwnerID string `json:"owner_id,omitempty"`
	// application id of the group DM creator if it is bot-created
	ApplicationID string `json:"application_id,omitempty"`
	// id of the parent category for a channel (each parent category can contain up to 50 channels)
	ParentID string `json:"parent_id,omitempty"`
	// when the last pinned message was pinned. This may be null in events such as GUILD_CREATE when a
	// message is not pinned.
	LastPinTimestamp string `json:"last_pin_timestamp,omitempty"`
}
