package types

// ActivityTypes are the different types for an activity
type ActivityTypes int

const (
	// ActivityTypeGame is the "Playing {name}" activity
	ActivityTypeGame ActivityTypes = 0
	// ActivityTypeStreaming is the "Streaming {details}" activity
	// The streaming type currently only supports Twitch and YouTube.
	// Only https://twitch.tv/ and https://youtube.com/ urls will work.
	ActivityTypeStreaming = 1
	// ActivityTypeListening is the "Listening to {name}" activity
	ActivityTypeListening = 2

	/* yes, there is no 3 */

	// ActivityTypeCustom is the "{emoji} {name}" activity
	ActivityTypeCustom = 4
	// ActivityTypeCompeting is the "Competing in {name}" activity
	ActivityTypeCompeting = 5
)
