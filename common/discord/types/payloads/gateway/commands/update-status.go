package commands

import (
	"github.com/discordnova/nova/common/discord/types/structures"
	"github.com/discordnova/nova/common/discord/types/types"
)

// GatewayCommandUpdateStatusPayload is sent by the client to indicate a presence or status update.
type GatewayCommandUpdateStatusPayload struct {
	// 	unix time (in milliseconds) of when the client went idle, or null if the client is not idle
	Since int `json:"since,omitempty"`
	// null, or the user's activities
	Activities []structures.Activity `json:"activities,omitempty"`
	// the user's new status
	Status types.UpdateStatusStatuses `json:"status"`
	// whether or not the client is afk
	AFK bool `json:"afk"`
}
