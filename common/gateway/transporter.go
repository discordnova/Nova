package gateway

type PushData struct {
	Data []byte
	Name string
}

// Transporter is the base interface for the transportation layer of Nova
type Transporter interface {
	PushChannel() chan PushData
	PullChannel() chan []byte
}
