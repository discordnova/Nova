package types

// UserFlags is the flags of a User
type UserFlags int

// Valid UserFlags
const (
	UserFlagNone                      UserFlags = 0
	UserFlagDiscordEmployee                     = 1 << 0
	UserFlagPartneredServerOwner                = 1 << 1
	UserFlagHypeSquadEvents                     = 1 << 2
	UserFlagBugHunterLevel1                     = 1 << 3
	UserFlagHouseBravery                        = 1 << 6
	UserFlagHouseBrilliance                     = 1 << 7
	UserFlagHouseBalance                        = 1 << 8
	UserFlagEarlySupporter                      = 1 << 9
	UserFlagTeamUser                            = 1 << 10
	UserFlagSystem                              = 1 << 12
	UserFlagBugHunterLevel2                     = 1 << 14
	UserFlagVerifiedBot                         = 1 << 16
	UserFlagEarlyVerifiedBotDeveloper           = 1 << 17
)
