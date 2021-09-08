package events

import "github.com/discordnova/nova/common/discord/types/structures"

// GatewayEventGuildMembersChunkPayload is sent in response to Guild Request Members.
// You can use the chunk_index and chunk_count to calculate how many chunks are left for your request.
type GatewayEventGuildMembersChunkPayload struct {
	// the id of the guild
	GuildID string `json:"guild_id"`
	// set of guild members
	Members []structures.GuildMember `json:"members"`
	// the chunk index in the expected chunks for this response (0 <= chunk_index < chunk_count)
	ChunkIndex int `json:"chunk_index"`
	// the total number of expected chunks for this response
	ChunkCount int `json:"chunk_count"`
	// if passing an invalid id to REQUEST_GUILD_MEMBERS, it will be returned here
	NotFound []string `json:"not_found,omitempty"`
	// if passing true to REQUEST_GUILD_MEMBERS, presences of the returned members will be here
	Presences []structures.Presence `json:"presences"`
	// 	the nonce used in the Guild Members Request
	Nonce string `json:"nonce"`
}
