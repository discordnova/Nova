package structures

// ActivityAssets is a representation of the assets object sent in an Activity
type ActivityAssets struct {
	// the id for a large asset of the activity, usually a snowflake
	LargeImage string `json:"large_image,omitempty"`
	// text displayed when hovering over the large image of the activity
	LargeText string `json:"large_text,omitempty"`
	// the id for a small asset of the activity, usually a snowflake
	SmallImage string `json:"small_image,omitempty"`
	// text displayed when hovering over the small image of the activity
	SmallText string `json:"small_text,omitempty"`
}
