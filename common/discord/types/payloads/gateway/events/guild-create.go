package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildCreatePayload can be sent in three different scenarios:
// When a user is initially connecting,
// to lazily load and back fill information for all unavailable guilds sent in the Ready event.
// Guilds that are unavailable due to an outage will send a Guild Delete event.
// When a Guild becomes available again to the client.
// When the current user joins a new Guild.
// The inner payload is a guild object, with all the extra fields specified. (Nothing on the doc)
type GatewayEventGuildCreatePayload structures.Guild
