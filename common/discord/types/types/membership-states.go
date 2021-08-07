package types

// MembershipStates are the different state of a membership in a Team
type MembershipStates int

const (
	// MembershipStateInvited is the state of a pending invitation
	MembershipStateInvited MembershipStates = 1
	// MembershipStateAccepted is the state of an accepted invitation
	MembershipStateAccepted = 2
)
