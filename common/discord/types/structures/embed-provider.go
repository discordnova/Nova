package structures

// EmbedProvider represents the provider object of an embed
type EmbedProvider struct {
	// name of provider
	Name string `json:"name,omitempty"`
	// url of provider
	URL string `json:"url,omitempty"`
}
