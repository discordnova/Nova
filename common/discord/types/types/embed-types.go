package types

// EmbedTypes are the different types of a Embed
type EmbedTypes string

const (
	// EmbedTypeRich is a generic embed rendered from embed attributes
	EmbedTypeRich EmbedTypes = "rich"
	// EmbedTypeImage is an image embed
	EmbedTypeImage = "image"
	// EmbedTypeVideo is a video embed
	EmbedTypeVideo = "video"
	// EmbedTypeGIFV is an animated gif image embed rendered as a video embed
	EmbedTypeGIFV = "gifv"
	// EmbedTypeArticle is an article embed
	EmbedTypeArticle = "article"
	// EmbedTypeLink is a link embed
	EmbedTypeLink = "link"
)
