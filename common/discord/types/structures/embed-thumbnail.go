package structures

// EmbedThumbnail represents the thumbnail object of an embed
type EmbedThumbnail struct {
	// source url of thumbnail (only supports http(s) and attachments)
	URL string `json:"url,omitempty"`
	// a proxied url of the thumbnail
	ProxyURL string `json:"proxy_url,omitempty"`
	// height of thumbnail
	Height int `json:"height,omitempty"`
	// width of thumbnail
	Width int `json:"width,omitempty"`
}
