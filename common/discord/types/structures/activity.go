package structures

import "github.com/discordnova/nova/common/discord/types/types"

// Activity represents a presence's activity
// Bots are only able to send name, type, and optionally url.
type Activity struct {
	// the activity's name
	Name string `json:"name"`
	// 	activity type
	Type int `json:"type"`
	// stream url, is validated when type is 1
	URL string `json:"url,omitempty"`
	// unix timestamp of when the activity was added to the user's session
	CreatedAt int `json:"created_at"`
	// unix timestamps for start and/or end of the game
	Timestamps ActivityTimestamps `json:"timestamps,omitempty"`
	// application id for the game
	ApplicationID string `json:"application_id,omitempty"`
	// what the player is currently doing
	Details string `json:"details,omitempty"`
	// the user's current party status
	State string `json:"state,omitempty"`
	// the emoji used for a custom status
	Emoji ActivityEmoji `json:"emoji,omitempty"`
	// information for the current party of the player
	Party ActivityParty `json:"party,omitempty"`
	// images for the presence and their hover texts
	Assets ActivityAssets `json:"assets,omitempty"`
	// secrets for Rich Presence joining and spectating
	Secrets ActivitySecrets `json:"secrets,omitempty"`
	// whether or not the activity is an instanced game session
	Instance bool `json:"instance,omitempty"`
	// activity flags ORd together, describes what the payload includes
	Flags types.ActivityFlags `json:"flags,omitempty"`
}
