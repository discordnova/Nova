package gateway

// This is used to map the discord dispatch events
// to internal names used by the event broker.

// TODO: Update the types of events for the gateway v9
var EventNames = map[string]string{
	"READY":     "gateway.ready",
	"RESUMED":   "gateway.resumed",
	"RECONNECT": "gateway.reconnect",

	"CHANNEL_CREATE": "channel.create",
	"CHANNEL_UPDATE": "channel.update",
	"CHANNEL_DELETE": "channel.delete",

	"GUILD_CREATE": "guild.create",
	"GUILD_UPDATE": "guild.update",
	"GUILD_DELETE": "guild.delete",

	"GUILD_BAN_ADD":    "guild.ban.add",
	"GUILD_BAN_REMOVE": "guild.ban.remove",

	"GUILD_EMOJIS_UPDATE":       "guild.emojis_update",
	"GUILD_INTEGRATIONS_UPDATE": "guild.integrations_update",

	"GUILD_MEMBER_ADD":     "guild.member.add",
	"GUILD_MEMBER_REMOVE":  "guild.member.remove",
	"GUILD_MEMBERS_UPDATE": "guild.member.update",

	"GUILD_MEMBERS_CHUNK": "guild_members_chunk",

	"GUILD_ROLE_CREATE": "guild.role.create",
	"GUILD_ROLE_UPDATE": "guild.role.update",
	"GUILD_ROLE_DELETE": "guild.role.delete",

	"INVITE_CREATE": "guild.invite.create",
	"INVITE_DELETE": "guild.invite.guild",

	"MESSAGE_CREATE":                "message.create",
	"MESSAGE_UPDATE":                "message.update",
	"MESSAGE_DELETE_BULK":           "message.delete_bulk",
	"MESSAGE_REACTION_ADD":          "message.reactions.add",
	"MESSAGE_REACTION_REMOVE":       "message.reactions.remove",
	"MESSAGE_REACTION_REMOVE_ALL":   "message.reactions.remove_all",
	"MESSAGE_REACTION_REMOVE_EMOJI": "message.reactions.remove_emoji",
	"PRESENCE_UPDATE":               "users.presence_update",
	"TYPING_START":                  "message.typing_start",
	"USER_UPDATE":                   "users.update",
	"VOICE_STATE_UPDATE":            "voice.state_update",
	"VOICE_SERVER_UPDATE":           "voice.server_update",
	"WEBHOOKS_UPDATE":               "guild.webhooks.update",
}
