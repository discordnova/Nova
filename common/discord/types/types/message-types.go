package types

// MessageTypes are the types of a message
type MessageTypes int

//
const (
	MessageTypeDefault                           MessageTypes = 0
	MessageTypeRecipientAdd                                   = 1
	MessageTypeRecipientRemove                                = 2
	MessageTypeCall                                           = 3
	MessageTypeChannelNameChange                              = 4
	MessageTypeChannelIconChange                              = 5
	MessageTypeChannelPinnedMessage                           = 6
	MessageTypeGuildMemberJoin                                = 7
	MessageTypeUserPremiumGuildSubscription                   = 8
	MessageTypeUserPremiumGuildSubscriptionTier1              = 9
	MessageTypeUserPremiumGuildSubscriptionTier2              = 10
	MessageTypeUserPremiumGuildSubscriptionTier3              = 11
	MessageTypeChannelFollowAdd                               = 12
	MessageTypeGuildDiscoveryDisqualified                     = 14
	MessageTypeGuildDiscoveryRequalified                      = 15
	MessageTypeReply                                          = 19
	MessageTypeApplicationCommand                             = 20
)
