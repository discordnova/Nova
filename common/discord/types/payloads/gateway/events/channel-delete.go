package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventChannelDeletePayload is sent when a channel relevant to the current user is deleted.
// The inner payload is a channel object.
type GatewayEventChannelDeletePayload structures.Channel
