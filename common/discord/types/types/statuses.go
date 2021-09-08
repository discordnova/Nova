package types

// Statuses is a Presence status for a user
type Statuses string

const (
	// StatusOnline is the status for an online user
	StatusOnline Statuses = "online"
	// StatusIdle is the status for an idle user
	StatusIdle = "idle"
	// StatusDND is the status for a user in Do not Disturb
	StatusDND = "dnd"
	// StatusOffline is the status for an offline or invisible user
	StatusOffline = "offline"
)
