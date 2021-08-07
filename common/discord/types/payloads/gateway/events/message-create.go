package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventMessageCreatePayload is sent when a message is created. The inner payload is a message object.
type GatewayEventMessageCreatePayload structures.Message
