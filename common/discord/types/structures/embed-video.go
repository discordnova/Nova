package structures

// EmbedVideo represents the video object of an embed
type EmbedVideo struct {
	// source url of video
	URL string `json:"url,omitempty"`
	// a proxied url of the video
	ProxyURL string `json:"proxy_url,omitempty"`
	// height of video
	Height int `json:"height,omitempty"`
	// width of video
	Width int `json:"width,omitempty"`
}
