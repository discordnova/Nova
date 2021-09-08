package structures

// IdentifyConnectionProperties is the identify connection properties object of an identify command
type IdentifyConnectionProperties struct {
	// your operating system
	OS string `json:"$os"`
	// your library name
	Browser string `json:"$browser"`
	// 	your library name
	Device string `json:"$device"`
}
