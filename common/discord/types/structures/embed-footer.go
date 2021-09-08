package structures

// EmbedFooter represents the footer of an embed
type EmbedFooter struct {
	// footer text
	Text string `json:"text"`
	// url of footer icon (only supports http(s) and attachments)
	IconURL string `json:"icon_url,omitempty"`
	// 	a proxied url of footer icon
	ProxyIconURL string `json:"proxy_icon_url,omitempty"`
}
