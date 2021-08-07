package types

// VerificationLevels is the verification level of a guild
type VerificationLevels int

const (
	// VerificationLevelNone needs nothing
	VerificationLevelNone VerificationLevels = 0
	// VerificationLevelLow needs members to have a verified email
	VerificationLevelLow = 1
	// VerificationLevelMedium needs members to be registered on Discord for
	// longer than 5 minutes
	VerificationLevelMedium = 2
	// VerificationLevelHigh needs members to be a member of the server for
	// longer than 10 minutes
	VerificationLevelHigh = 3
	// VerificationLevelVeryHigh needs members to have a verified phone number
	VerificationLevelVeryHigh = 4
)
