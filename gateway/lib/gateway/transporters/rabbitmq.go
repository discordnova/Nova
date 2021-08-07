package transporters

import (
	"time"

	"github.com/discordnova/nova/common/gateway"
	"github.com/rs/zerolog/log"
	"github.com/streadway/amqp"
)

type RabbitMqTransporter struct {
	pullChannel chan []byte
	pushChannel chan gateway.PushData
}

// NewRabbitMqTransporter creates a rabbitmq transporter using a given url
func NewRabbitMqTransporter(url string) (gateway.Transporter, error) {
	log.Info().Msg("connecting to the transporter using rabbitmq...")
	conn, err := amqp.Dial(url)

	if err != nil {
		return nil, err
	}

	send, err := conn.Channel()

	if err != nil {
		return nil, err
	}

	err = send.ExchangeDeclare(
		"nova_gateway_dispatch",
		"topic",
		true,
		false,
		false,
		true,
		nil,
	)

	if err != nil {
		return nil, err
	}

	pullChannel, pushChannel := make(chan []byte), make(chan gateway.PushData)

	go func() {
		for {
			data := <-pushChannel
			send.Publish(
				"nova_gateway_dispatch",
				data.Name,
				false,
				false,
				amqp.Publishing{
					Priority:  1,
					Timestamp: time.Now(),
					Type:      data.Name,
					Body:      data.Data,
				},
			)
		}
	}()

	return &RabbitMqTransporter{
		pullChannel: pullChannel,
		pushChannel: pushChannel,
	}, nil
}

func (t RabbitMqTransporter) PushChannel() chan gateway.PushData {
	return t.pushChannel
}
func (t RabbitMqTransporter) PullChannel() chan []byte {
	return t.pullChannel
}
