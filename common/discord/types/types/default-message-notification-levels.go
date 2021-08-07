package types

// DefaultMessageNotificationLevels is the notification level for a guild
type DefaultMessageNotificationLevels int

const (
	// DefaultMessageNotificationLevelAllMessages notify on every message
	DefaultMessageNotificationLevelAllMessages DefaultMessageNotificationLevels = 0
	// DefaultMessageNotificationLevelOnlyMentions notify only on mentions
	DefaultMessageNotificationLevelOnlyMentions = 1
)
