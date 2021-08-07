package types

// PremiumTiers are the different tiers of a premium on a guild (aka Boosts)
type PremiumTiers int

const (
	// PremiumTierNone is the tier with no boosts
	PremiumTierNone PremiumTiers = 0
	// PremiumTierTier1 is the first boosts tier
	PremiumTierTier1 = 1
	// PremiumTierTier2 is the second boosts tier
	PremiumTierTier2 = 2
	// PremiumTierTier3 is the third boosts tier
	PremiumTierTier3 = 3
)
