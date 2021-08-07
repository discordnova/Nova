package structures

import "github.com/discordnova/nova/common/discord/types/types"

// Guild is the representation of a Discord Guild, AKA server
type Guild struct {
	// ID is the guild id
	ID string `json:"id"`
	// Name is the guild name (2-100 characters, excluding trailing and
	// leading whitespace)
	Name string `json:"name"`
	// Icon is the icon hash
	Icon string `json:"icon,omitempty"`
	// IconHash is the icon hash, returned when in the template object
	IconHash string `json:"icon_hash,omitempty"`
	// Splash is the splash hash
	Splash string `json:"splash,omitempty"`
	// DiscoverySplash is 	discovery splash hash; only present for
	// guilds with the "DISCOVERABLE" feature
	DiscoverySplash string `json:"discovery_splash,omitempty"`
	// Owner is true if the user is the owner of the guild.
	// It is only sent when using the GET Current User Guilds endpoint
	// and is relative to the requested user
	Owner bool `json:"owner,omitempty"`
	// OwnerID is the id of owner
	OwnerID string `json:"owner_id,omitempty"`
	// Permissions are the total permissions for the user in the guild
	// (excludes overrides)
	// It is only sent when using the GET Current User Guilds endpoint
	// and is relative to the requested user
	Permissions string `json:"permissions,omitempty"`
	// Region is the voice region id for the guild
	Region string `json:"region"`
	// AFKChannelID is the id of the afk channel
	AFKChannelID string `json:"afk_channel_id,omitempty"`
	// AFKTimeout is the afk timeout in seconds
	AFKTimeout int `json:"afk_timeout"`
	// true if the server widget is enabled
	WidgetEnabled bool `json:"widget_enabled,omitempty"`
	// the channel id that the widget will generate an invite to, or null if set to no invite
	WidgetChannelID string `json:"widget_channel_id,omitempty"`
	// verification level required for the guild
	VerificationLevel types.VerificationLevels `json:"verification_level"`
	// default message notifications level
	DefaultMessageNotifications types.DefaultMessageNotificationLevels `json:"default_message_notifications"`
	// explicit content filter level
	ExplicitContentFilter types.ExplicitContentFilterLevels `json:"explicit_content_filter"`
	// roles in the guild
	Roles []Role `json:"roles"`
	// custom guild emojis
	Emojis []Emoji `json:"emojis"`
	// enabled guild features
	Features []types.GuildFeatures `json:"features"`
	// required MFA level for the guild
	MFALevel types.MFALevels `json:"mfa_level"`
	// application id of the guild creator if it is bot-created
	ApplicationID string `json:"application_id,omitempty"`
	// the id of the channel where guild notices such as welcome messages and boost events are posted
	SystemChannelID string `json:"system_channel_id,omitempty"`
	// system channel flags
	SystemChannelFlags types.SystemChannelFlags `json:"system_channel_flags,omitempty"`
	// 	the id of the channel where Community guilds can display rules and/or guidelines
	RulesChannelID string `json:"rules_channel_id,omitempty"`
	// when this guild was joined at
	// This field is only sent within the GUILD_CREATE event
	JoinedAt string `json:"joined_at,omitempty"`
	// true if this is considered a large guild
	// This field is only sent within the GUILD_CREATE event
	Large bool `json:"large,omitempty"`
	// true if this guild is unavailable due to an outage
	// This field is only sent within the GUILD_CREATE event
	Unavailable bool `json:"unavailable,omitempty"`
	// total number of members in this guild
	// This field is only sent within the GUILD_CREATE event
	MemberCount int `json:"member_count,omitempty"`
	// states of members currently in voice channels; lacks the guild_id key
	// This field is only sent within the GUILD_CREATE event
	VoiceStates []VoiceState `json:"voice_states,omitempty"`
	// users in the guild
	// This field is only sent within the GUILD_CREATE event
	Members []GuildMember `json:"members,omitempty"`
	// channels in the guild
	// This field is only sent within the GUILD_CREATE event
	Channels []Channel `json:"channels,omitempty"`
	// presences of the members in the guild, will only include non-offline members if the size is greater than
	// large threshold
	// This field is only sent within the GUILD_CREATE event
	Presences []Presence `json:"presences,omitempty"`
	// the maximum number of presences for the guild (the default value, currently 25000,
	// is in effect when null is returned)
	MaxPresences int `json:"max_presences,omitempty"`
	// the maximum number of members for the guild
	MaxMembers int `json:"max_members,omitempty"`
	// the vanity url code for the guild
	VanityURLCode string `json:"vanity_url_code,omitempty"`
	// the description for the guild, if the guild is discoverable
	Description string `json:"description,omitempty"`
	// banner hash
	Banner string `json:"banner,omitempty"`
	// premium tier (Server Boost level)
	PremiumTier types.PremiumTiers `json:"premium_tier"`
	// the number of boosts this guild currently has
	PremiumSubscriptionCount int `json:"premium_subscription_count,omitempty"`
	// the preferred locale of a Community guild; used in server discovery and notices from Discord; defaults to "en-US"
	PreferredLocale string `json:"preferred_locale"`
	// the id of the channel where admins and moderators of Community guilds receive notices from Discord
	PublicUpdatesChannelID string `json:"public_updates_channel_id,omitempty"`
	// the maximum amount of users in a video channel
	MaxVideoChannelUsers int `json:"max_video_channel_users,omitempty"`
	// approximate number of members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
	ApproximateMemberCount int `json:"approximate_member_count,omitempty"`
	// approximate number of non-offline members in this guild, returned from the GET /guilds/<id> endpoint when
	// with_counts is true
	ApproximatePresenceCount int `json:"approximate_presence_count,omitempty"`
	// the welcome screen of a Community guild, shown to new members, returned when in the invite object
	WelcomeScreen WelcomeScreen `json:"welcome_screen,omitempty"`
}
