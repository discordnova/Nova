package structures

// EmbedAuthor represents the author object of an embed
type EmbedAuthor struct {
	// name of author
	Name string `json:"name,omitempty"`
	// url of author
	URL string `json:"url,omitempty"`
	// url of author icon (only supports http(s) and attachments)
	IconURL string `json:"icon_url,omitempty"`
	// a proxied url of author icon
	ProxyIconURL string `json:"proxy_icon_url,omitempty"`
}
