package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildDeletePayload is sent when a guild becomes or was already unavailable due to an outage,
// or when the user leaves or is removed from a guild.
// The inner payload is an unavailable guild object.
// If the unavailable field is not set, the user was removed from the guild.
type GatewayEventGuildDeletePayload structures.Guild
