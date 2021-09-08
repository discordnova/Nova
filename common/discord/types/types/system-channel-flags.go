package types

// SystemChannelFlags are the flags of a channel
type SystemChannelFlags int

const (
	// SystemChannelFlagSuppressJoinNotifications is the flag when a channel suppresses member join notifications
	SystemChannelFlagSuppressJoinNotifications SystemChannelFlags = 1 << 0
	// SystemChannelFlagSuppressPremiumSubscriptions is the flag when a channel suppresses server boost notifications
	SystemChannelFlagSuppressPremiumSubscriptions = 1 << 1
)
