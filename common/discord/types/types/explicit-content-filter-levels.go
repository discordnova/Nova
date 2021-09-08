package types

// ExplicitContentFilterLevels is the level for the Explicit Content
type ExplicitContentFilterLevels int

const (
	// ExplicitContentFilterLevelDisabled disables the scan
	ExplicitContentFilterLevelDisabled = 0
	// ExplicitContentFilterLevelMembersWithoutRoles scans for members without roles
	ExplicitContentFilterLevelMembersWithoutRoles = 1
	// ExplicitContentFilterLevelAllMembers scans for anyone
	ExplicitContentFilterLevelAllMembers = 2
)
