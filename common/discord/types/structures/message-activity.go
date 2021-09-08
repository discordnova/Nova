package structures

import "github.com/discordnova/nova/common/discord/types/types"

// MessageActivity is the message activity
type MessageActivity struct {
	// type of message activity
	Type types.MessageActivityTypes `json:"type"`
	// party_id from a Rich Presence event
	PartyID string `json:"party_id"`
}
