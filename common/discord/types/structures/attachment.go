package structures

// Attachment is the representation of a message attachment
type Attachment struct {
	// attachment id
	ID string `json:"id"`
	// name of file attached
	Filename string `json:"filename"`
	// size of file in bytes
	Size int `json:"size"`
	// source url of file
	URL string `json:"url"`
	// a proxied url of file
	ProxyURL string `json:"proxy_url"`
	// height of file (if image)
	Height int `json:"height,omitempty"`
	// width of file (if image)
	Width int `json:"width,omitempty"`
}
