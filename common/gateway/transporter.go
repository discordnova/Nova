package gateway

// Transporter is the base interface to push events to
type Transporter interface {
	PushDispatchEvent(name string, data []byte) error
	PushEventCache(name string, data []byte) error
}
