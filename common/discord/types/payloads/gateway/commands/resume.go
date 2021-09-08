package commands

// GatewayCommandResumePayload is used to replay missed events when a disconnected client resumes
type GatewayCommandResumePayload struct {
	// 	session token
	Token string `json:"token"`
	// 	session id
	SessionID string `json:"session_id"`
	// last sequence number received
	Seq int64 `json:"seq"`
}
