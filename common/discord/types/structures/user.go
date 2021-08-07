package structures

import "github.com/discordnova/nova/common/discord/types/types"

// User represents the User Structure sent by the Discord API
type User struct {
	ID            string             `json:"id"`
	Username      string             `json:"username"`
	Discriminator string             `json:"discriminator"`
	Avatar        string             `json:"avatar,omitempty"`
	Bot           bool               `json:"bot,omitempty"`
	System        bool               `json:"system,omitempty"`
	MFAEnabled    bool               `json:"mfa_enabled,omitempty"`
	Locale        string             `json:"locale,omitempty"`
	Verified      bool               `json:"verified,omitempty"`
	Email         string             `json:"email,omitempty"`
	Flags         types.UserFlags    `json:"flags,omitempty"`
	PremiumType   types.PremiumTypes `json:"premium_type,omitempty"`
	PublicFlags   types.UserFlags    `json:"public_flags,omitempty"`
}
