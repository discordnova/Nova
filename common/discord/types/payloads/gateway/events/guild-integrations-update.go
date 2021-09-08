package events

// GatewayEventGuildIntegrationsUpdatePayload is sent when a guild integration is updated.
type GatewayEventGuildIntegrationsUpdatePayload struct {
	// id of the guild whose integrations were updated
	GuildID string `json:"guild_id"`
}
