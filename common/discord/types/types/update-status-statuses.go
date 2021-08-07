package types

// UpdateStatusStatuses are the statuses used in a Update Status gateway command
type UpdateStatusStatuses string

//
const (
	// UpdateStatusStatusOnline is the online status
	UpdateStatusStatusOnline UpdateStatusStatuses = "online"
	// UpdateStatusStatusDoNotDisturb is the Do Not Disturb status
	UpdateStatusStatusDoNotDisturb = "dnd"
	// UpdateStatusStatusIdle is the AFK status
	UpdateStatusStatusIdle = "idle"
	// UpdateStatusStatusInvisible is the Invisible status, shown as offline
	UpdateStatusStatusInvisible = "invisible"
	// UpdateStatusStatusOffline is the offline status
	UpdateStatusStatusOffline = "offline"
)
