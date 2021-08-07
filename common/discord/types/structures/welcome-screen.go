package structures

// WelcomeScreen is a representation of a Guild welcome screen
type WelcomeScreen struct {
	// the server description shown in the welcome screen
	Description string `json:"description,omitempty"`
	// the channels shown in the welcome screen, up to 5
	WelcomeChannels []WelcomeScreenChannel `json:"welcome_channels"`
}
