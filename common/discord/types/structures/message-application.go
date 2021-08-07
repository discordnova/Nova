package structures

// MessageApplication is the application object of a message
type MessageApplication struct {
	// id of the application
	ID string `json:"id"`
	// id of the embed's image asset
	CoverImage string `json:"cover_image,omitempty"`
	// application's description
	Description string `json:"description"`
	// id of the application's icon
	Icon string `json:"icon,omitempty"`
	// name of the application
	Name string `json:"name"`
}
