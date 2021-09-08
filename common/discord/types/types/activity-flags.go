package types

// ActivityFlags are the flags of an activity
type ActivityFlags int

const (
	// ActivityFlagInstance is the flag for instanced activities
	ActivityFlagInstance ActivityFlags = 1 << 0
	// ActivityFlagJoin is the flag for activities we can join
	ActivityFlagJoin = 1 << 1
	// ActivityFlagSpectate is the flag for activities we can spectate
	ActivityFlagSpectate = 1 << 2
	// ActivityFlagJoinRequest is the flag for activities we can request to join
	ActivityFlagJoinRequest = 1 << 3
	// ActivityFlagSync is the flag for sync activities
	ActivityFlagSync = 1 << 4
	// ActivityFlagPlay is the flag for playable activities
	ActivityFlagPlay = 1 << 5
)
