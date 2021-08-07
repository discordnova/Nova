package structures

// RoleTags is the tag a role has
type RoleTags struct {
	BotID         string `json:"bot_id,omitempty"`
	IntegrationID string `json:"integration_id,omitempty"`
	// ALWAYS NULL YET
	PremiumSubscriber struct{} `json:"premium_subscriber"`
}
