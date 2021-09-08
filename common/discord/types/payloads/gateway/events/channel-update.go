package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventChannelUpdatePayload is sent when a channel is updated. The inner payload is a channel object.
// This is not sent when the field last_message_id is altered.
// To keep track of the last_message_id changes, we should listen for Message Create events.
type GatewayEventChannelUpdatePayload structures.Channel
