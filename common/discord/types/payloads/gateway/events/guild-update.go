package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildUpdatePayload is sent when a guild is updated. The inner payload is a guild object.
type GatewayEventGuildUpdatePayload structures.Guild
