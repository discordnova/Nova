package structures

import "github.com/discordnova/nova/common/discord/types/types"

// Embed is the representation of an Embed
type Embed struct {
	// title of embed
	Title string `json:"title,omitempty"`
	// type of embed (always "rich" for webhook embeds)
	Type types.EmbedTypes `json:"type,omitempty"`
	// description of embed
	Description string `json:"description,omitempty"`
	// url of embed
	URL string `json:"url,omitempty"`
	// timestamp of embed content
	Timestamp string `json:"timestamp,omitempty"`
	// color code of the embed
	Color int `json:"color,omitempty"`
	// footer information
	Footer EmbedFooter `json:"footer,omitempty"`
	// image information
	Image EmbedImage `json:"image,omitempty"`
	// thumbnail information
	Thumbnail EmbedThumbnail `json:"thumbnail,omitempty"`
	// video information
	Video EmbedVideo `json:"video,omitempty"`
	// provider information
	Provider EmbedProvider `json:"provider,omitempty"`
	// author information
	Author EmbedAuthor `json:"author,omitempty"`
	// fields information
	Fields []EmbedField `json:"fields,omitempty"`
}
