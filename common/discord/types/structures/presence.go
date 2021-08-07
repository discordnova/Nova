package structures

import "github.com/discordnova/nova/common/discord/types/types"

// Presence is the representation of a user's presence on a guild.
type Presence struct {
	// the user presence is being updated for
	User User `json:"user"`
	// id of the guild
	GuildID string `json:"guild_id"`
	// either "idle", "dnd", "online", or "offline"
	Status types.Statuses `json:"status"`
	// user's current activities
	Activities []Activity `json:"activities"`
	// user's platform-dependent status
	ClientStatus ClientStatus `json:"client_status"`
}
