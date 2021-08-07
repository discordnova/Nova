package events

import (
	"github.com/discordnova/nova/common/discord/types/structures"
	"github.com/discordnova/nova/common/discord/types/types"
)

// GatewayEventPresenceUpdatePayload is sent when a user's presence or info, such as name or avatar, is updated.
// If you are using Gateway Intents, you must specify the GUILD_PRESENCES intent in order to receive
// Presence Update events
// The user object within this event can be partial, the only field which must be sent is the id field,
// everything else is optional. Along with this limitation, no fields are required,
// and the types of the fields are not validated.
// Your client should expect any combination of fields and types within this event.
type GatewayEventPresenceUpdatePayload struct {
	// the user presence is being updated for
	User structures.User `json:"user"`
	// id of the guild
	GuildID string `json:"guild_id"`
	// either "idle", "dnd", "online", or "offline"
	Status types.Statuses `json:"status"`
	// user's current activities
	Activities []structures.Activity `json:"activities"`
	// user's platform-dependent status
	ClientStatus types.ClientStatuses `json:"client_status"`
}
