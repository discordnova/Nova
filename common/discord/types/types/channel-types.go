package types

// ChannelTypes are the different types of channel
type ChannelTypes int

const (
	// ChannelTypeGuildText is a text channel within a server
	ChannelTypeGuildText ChannelTypes = 0
	// ChannelTypeDM is a direct message between users
	ChannelTypeDM = 1
	// ChannelTypeGuildVoice is a voice channel within a server
	ChannelTypeGuildVoice = 2
	// ChannelTypeGroupDM is a direct message between multiple users
	ChannelTypeGroupDM = 3
	// ChannelTypeGuildCategory is an organizational category that contains up to 50 channels
	ChannelTypeGuildCategory = 4
	// ChannelTypeGuildNews is a channel that users can follow and cross post into their own server
	ChannelTypeGuildNews = 5
	// ChannelTypeGuildStore is a channel in which game developers can sell their game on Discord
	ChannelTypeGuildStore = 6
)
