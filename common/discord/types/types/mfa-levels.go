package types

// MFALevels are the different level of MFA (a2f)
type MFALevels int

const (
	// MFALevelNone is the level where MFA is not required
	MFALevelNone MFALevels = 0
	// MFALevelElevated is the level where MFA is required
	MFALevelElevated = 1
)
