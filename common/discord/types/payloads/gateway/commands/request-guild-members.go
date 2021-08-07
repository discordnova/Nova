package commands

// GatewayCommandRequestGuildMembersPayload is used o request all members for a guild or a list of guilds.
// When initially connecting,the gateway will only send offline members if a guild has less than the large_threshold
// members (value in the Gateway Identify).
// If a client wishes to receive additional members, they need to explicitly request them via this operation.
// The server will send Guild Members Chunk events in response with up to 1000 members per chunk until all members
// that match the request have been sent.
// Due to our privacy and infrastructural concerns with this feature, there are some limitations that apply:
// GUILD_PRESENCES intent is required to set presences = true. Otherwise, it will always be false
// GUILD_MEMBERS intent is required to request the entire member list—(query=‘’, limit=0<=n)
// You will be limited to requesting 1 guild_id per request
// Requesting a prefix (query parameter) will return a maximum of 100 members
// Requesting user_ids will continue to be limited to returning 100 members
type GatewayCommandRequestGuildMembersPayload struct {
	// id of the guild to get members for
	// Required: true
	GuildID string `json:"guild_id"`
	// string that username starts with, or an empty string to return all members
	// Required: one of query or user_ids
	Query string `json:"query,omitempty"`
	// maximum number of members to send matching the query; a limit of 0 can be used with an empty string query to return
	// all members
	// Required: true when specifying query
	Limit int `json:"limit,omitempty"`
	// used to specify if we want the presences of the matched members
	// Required: false
	Presences bool `json:"presences,omitempty"`
	// used to specify which users you wish to fetch
	// Required: one of query or user_ids
	UserIDs string `json:"user_ids,omitempty"`
	// 	nonce to identify the Guild Members Chunk response
	// Required: false
	Nonce string `json:"nonce,omitempty"`
}
