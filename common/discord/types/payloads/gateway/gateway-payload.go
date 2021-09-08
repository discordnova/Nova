package gateway

import (
	"encoding/json"

	"github.com/discordnova/nova/common/discord/types/types"
)

// Payload is the base of each payload sent to the Gateway, everything
// else is in the d property.
type Payload struct {
	// Op is the opcode for the payload
	Op types.GatewayOpCodes `json:"op"`
	// D is the event data
	D json.RawMessage `json:"d"`
	// S is a sequence number used for resuming sessions and heartbeats
	S int64 `json:"s"`
	// T is the event name for this payload
	T string `json:"t"`
}
