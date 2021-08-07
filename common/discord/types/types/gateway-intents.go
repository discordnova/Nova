package types

// GatewayIntents are the Intents for the Gateway
type GatewayIntents int

//
const (
	GatewayIntentGuilds                 GatewayIntents = 1 << 0
	GatewayIntentGuildMembers                          = 1 << 1
	GatewayIntentGuildBans                             = 1 << 2
	GatewayIntentGuildEmojis                           = 1 << 2
	GatewayIntentGuildIntegrations                     = 1 << 4
	GatewayIntentGuildWebhooks                         = 1 << 5
	GatewayIntentGuildInvites                          = 1 << 6
	GatewayIntentGuildVoiceStates                      = 1 << 7
	GatewayIntentGuildPresences                        = 1 << 8
	GatewayIntentGuildMessages                         = 1 << 9
	GatewayIntentGuildMessageReactions                 = 1 << 10
	GatewayIntentGuildMessageTyping                    = 1 << 11
	GatewayIntentDirectMessages                        = 1 << 12
	GatewayIntentDirectMessageReactions                = 1 << 13
	GatewayIntentDirectMessageTyping                   = 1 << 14
)
