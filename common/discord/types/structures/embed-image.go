package structures

// EmbedImage represents the image object of an embed
type EmbedImage struct {
	// source url of image (only supports http(s) and attachments)
	URL string `json:"url,omitempty"`
	// a proxied url of the image
	ProxyURL string `json:"proxy_url,omitempty"`
	// height of image
	Height int `json:"height,omitempty"`
	// width of image
	Width int `json:"width,omitempty"`
}
