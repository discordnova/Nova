package types

// MessageActivityTypes are the types of a message activity
type MessageActivityTypes int

//
const (
	MessageActivityTypeJoin        MessageActivityTypes = 1
	MessageActivityTypeSpectate                         = 2
	MessageActivityTypeListen                           = 3
	MessageActivityTypeJoinRequest                      = 5
)
