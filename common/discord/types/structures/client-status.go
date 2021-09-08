package structures

import "github.com/discordnova/nova/common/discord/types/types"

// ClientStatus is the representation of a Client Status, for sessions indication
type ClientStatus struct {
	// the user's status set for an active desktop (Windows, Linux, Mac) application session
	Desktop types.ClientStatuses `json:"desktop,omitempty"`
	// the user's status set for an active mobile (iOS, Android) application session
	Mobile types.ClientStatuses `json:"mobile,omitempty"`
	// the user's status set for an active web (browser, bot account) application session
	Web types.ClientStatuses `json:"web,omitempty"`
}
