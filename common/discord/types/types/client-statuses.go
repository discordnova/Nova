package types

// ClientStatuses are the different client status for active sessions
type ClientStatuses string

const (
	// ClientStatusOnline is the status for online users
	ClientStatusOnline ClientStatuses = "online"
	// ClientStatusIdle is the status for idle sessions
	ClientStatusIdle ClientStatuses = "idle"
	// ClientStatusDnD is the status for do not disturb sessions
	ClientStatusDnD ClientStatuses = "dnd"
	/* There is no offline: https://discord.com/developers/docs/topics/gateway#client-status-object */
)
