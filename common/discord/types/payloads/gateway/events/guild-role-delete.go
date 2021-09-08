package events

// GatewayEventGuildRoleDeletePayload is sent when a guild role is updated.
type GatewayEventGuildRoleDeletePayload struct {
	// the id of the guild
	GuildID string `json:"guild_id"`
	// id of the role
	RoleID string `json:"role_id"`
}
