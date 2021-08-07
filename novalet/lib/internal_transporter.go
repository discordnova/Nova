package lib

import (
	"github.com/discordnova/nova/common/gateway"
	"github.com/rs/zerolog/log"
)

type InternalTransporter struct {
	pullChannel chan []byte
	pushChannel chan gateway.PushData
}

// NewRabbitMqTransporter creates a rabbitmq transporter using a given url
func NewInternalTransporter() (gateway.Transporter, error) {
	log.Info().Msg("using the memory transporter")

	pullChannel, pushChannel := make(chan []byte), make(chan gateway.PushData)

	go func() {
		for {
			// TODO(matthieu): Implement push channel for the internal transporter.
			<-pushChannel
		}
	}()

	return &InternalTransporter{
		pullChannel: pullChannel,
		pushChannel: pushChannel,
	}, nil
}

func (t InternalTransporter) PushChannel() chan gateway.PushData {
	return t.pushChannel
}
func (t InternalTransporter) PullChannel() chan []byte {
	return t.pullChannel
}
