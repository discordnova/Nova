package structures

import "github.com/discordnova/nova/common/discord/types/types"

// Sticker represents a Discord message sticker
type Sticker struct {
	// 	id of the sticker
	ID string `json:"id"`
	// id of the pack the sticker is from
	PackID string `json:"pack_id"`
	// name of the sticker
	Name string `json:"name"`
	// description of the sticker
	Description string `json:"description"`
	// a comma-separated list of tags for the sticker
	Tags string `json:"tags,omitempty"`
	// sticker asset hash
	Asset string `json:"asset"`
	// sticker preview asset hash
	PreviewAsset string `json:"preview_asset,omitempty"`
	// type of sticker format
	FormatType types.StickerTypes `json:"format_type"`
}
