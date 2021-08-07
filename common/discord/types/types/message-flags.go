package types

// MessageFlags are the flags of a message
type MessageFlags int

const (
	// MessageFlagCrossPosted is used when this message has been published to subscribed channels (
	// via Channel Following)
	MessageFlagCrossPosted MessageFlags = 1 << 0
	// MessageFlagIsCrossPost is used when this message originated from a message in another channel (
	// via Channel Following)
	MessageFlagIsCrossPost = 1 << 1
	// MessageFlagSuppressEmbeds is used when not include any embeds when serializing this message
	MessageFlagSuppressEmbeds = 1 << 2
	// MessageFlagSourceMessageDeleted is used when the source message for this cross post has been deleted (
	// via Channel Following)
	MessageFlagSourceMessageDeleted = 1 << 3
	// MessageFlagUrgent is used when this message came from the urgent message system
	MessageFlagUrgent = 1 << 4
)
