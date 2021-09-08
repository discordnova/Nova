package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventMessageUpdatePayload is sent when a message is updated. The inner payload is a message object.
// Unlike creates, message updates may contain only a subset of the full message object payload
// (but will always contain an id and channel_id).
type GatewayEventMessageUpdatePayload structures.Message
