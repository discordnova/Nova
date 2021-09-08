package structures

// Application represents a Discord Application
type Application struct {
	// the id of the app
	ID string `json:"id"`
	// the name of the app
	Name string `json:"name"`
	// the icon hash of the app
	Icon string `json:"icon,omitempty"`
	// the description of the app
	Description string `json:"description"`
	// an array of rpc origin urls, if rpc is enabled
	RPCOrigins []string `json:"rpc_origins,omitempty"`
	// when false only app owner can join the app's bot to guilds
	BotPublic bool `json:"bot_public"`
	// when true the app's bot will only join upon completion of the full oauth2 code grant flow
	BotRequireCodeGrant bool `json:"bot_require_code_grant"`
	// partial user object containing info on the owner of the application
	Owner User `json:"owner"`
	// if this application is a game sold on Discord, this field will be the summary field for the store page of its sku
	Summary string `json:"summary"`
	// the base64 encoded key for the GameSDK's GetTicket
	VerifyKey string `json:"verify_key"`
	// if the application belongs to a team, this will be a list of the members of that team
	Team Team `json:"team"`
	// if this application is a game sold on Discord, this field will be the guild to which it has been linked
	GuildID string `json:"guild_id,omitempty"`
	// if this application is a game sold on Discord, this field will be the id of the "Game SKU" that is created,
	// if exists
	PrimarySKUID string `json:"primary_sku_id,omitempty"`
	// if this application is a game sold on Discord, this field will be the URL slug that links to the store page
	Slug string `json:"slug,omitempty"`
	// if this application is a game sold on Discord, this field will be the hash of the image on store embeds
	CoverImage string `json:"cover_image,omitempty"`
	// the application's public flags
	Flags int `json:"flags"`
}
