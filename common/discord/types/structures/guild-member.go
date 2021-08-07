package structures

// GuildMember is the representation of a Guild member
type GuildMember struct {
	// the user this guild member represents
	// The field user won't be included in the member object attached to MESSAGE_CREATE and MESSAGE_UPDATE gateway
	// events.
	User User `json:"user,omitempty"`
	// this users guild nickname
	Nick string `json:"nick"`
	// array of role object ids
	Roles []string `json:"roles"`
	// when the user joined the guild
	JoinedAt string `json:"joined_at"`
	// when the user started boosting the guild
	PremiumSince string `json:"premium_since,omitempty"`
	// whether the user is deafened in voice channels
	Deaf bool `json:"deaf"`
	// whether the user is muted in voice channels
	Mute bool `json:"mute"`
	// whether the user has not yet passed the guild's Membership Screening requirements
	// In GUILD_ events, pending will always be included as true or false. In non GUILD_ events which can only be
	// triggered by non-pending users, pending will not be included.
	Pending bool `json:"pending,omitempty"`
	// total permissions of the member in the channel, including overrides, returned when in the interaction object
	Permissions string `json:"permissions,omitempty"`
}
