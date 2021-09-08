package types

// GuildFeatures is the features enabled for a Guild
type GuildFeatures string

const (
	// GuildFeatureInviteSplash is the feature where a guild can set an invite splash background
	GuildFeatureInviteSplash GuildFeatures = "INVITE_SPLASH"
	// GuildFeatureVIPRegions is the feature where a guild has access to set 384kbps bitrate in voice
	//	// (previously VIP voice servers)
	GuildFeatureVIPRegions = "VIP_REGIONS"
	// GuildFeatureVanityURL is the feature where a guild has access to set a vanity URL
	GuildFeatureVanityURL = "VANITY_URL"
	// GuildFeatureVerified is the feature where a guild is verified
	GuildFeatureVerified = "VERIFIED"
	// GuildFeaturePartnered is the feature where a guild is partnered
	GuildFeaturePartnered = "PARTNERED"
	// GuildFeatureCommunity is the feature where a guild can enable welcome screen, Membership Screening,
	// and discovery, and receives community updates
	GuildFeatureCommunity = "COMMUNITY"
	// GuildFeatureCommerce is the feature where a guild has access to use commerce features (i.e. create store channels)
	GuildFeatureCommerce = "COMMERCE"
	// GuildFeatureNews is the feature where a guild has access to create news channels
	GuildFeatureNews = "NEWS"
	// GuildFeatureDiscoverable is the feature where a guild is able to be discovered in the directory
	GuildFeatureDiscoverable = "DISCOVERABLE"
	// GuildFeatureFeaturable is the feature where a guild is able to be featured in the directory
	GuildFeatureFeaturable = "FEATURABLE"
	// GuildFeatureAnimatedIcon is the feature where a guild has access to set an animated guild icon
	GuildFeatureAnimatedIcon = "ANIMATED_ICON"
	// GuildFeatureBanner is the feature where a guild has access to set a guild banner image
	GuildFeatureBanner = "BANNER"
	// GuildFeatureWelcomeScreenEnabled is the feature where a guild has enabled the welcome screen
	GuildFeatureWelcomeScreenEnabled = "WELCOME_SCREEN_ENABLED"
	// GuildFeatureMemberMemberVerificationGateEnabled is the feature where a guild has enabled Membership Screening
	GuildFeatureMemberMemberVerificationGateEnabled = "MEMBER_VERIFICATION_GATE_ENABLED"
	// GuildFeaturePreviewEnabled is the feature where a guild can be previewed before joining via
	// Membership Screening or the directory
	GuildFeaturePreviewEnabled = "PREVIEW_ENABLED"
)
