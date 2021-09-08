package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventChannelCreatePayload is sent when a new guild channel is created, relevant to the current user.
// The inner payload is a channel object.
type GatewayEventChannelCreatePayload structures.Channel
