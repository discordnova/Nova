package transporters

import (
	"time"

	"github.com/rs/zerolog/log"
	"github.com/streadway/amqp"
	"github.com/discordnova/nova/common/gateway"
)

type RabbitMqTransporter struct {
	connection  *amqp.Connection
	sendChannel *amqp.Channel
}

// NewRabbitMqTransporter creates a rabbitmq transporter using a given url
func NewRabbitMqTransporter(url string) (gateway.Transporter, error) {
	log.Info().Msg("connecting to the transporter using rabbitmq...")
	conn, err := amqp.Dial(url)

	if err != nil {
		return &RabbitMqTransporter{}, err
	}

	send, err := conn.Channel()

	if err != nil {
		return &RabbitMqTransporter{}, err
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
		return &RabbitMqTransporter{}, err
	}

	return RabbitMqTransporter{
		connection:  conn,
		sendChannel: send,
	}, nil
}

// PushDispatchEvent dispatches a general event to all the bot workers.
func (transporter RabbitMqTransporter) PushDispatchEvent(t string, data []byte) error {
	return transporter.sendChannel.Publish(
		"nova_gateway_dispatch",
		t,
		false,
		false,
		amqp.Publishing{
			Priority:  1,
			Timestamp: time.Now(),
			Type:      t,
			Body:      data,
		},
	)
}

// PushEventCache dispatches a cache specific events to all the cache workers.
func (transporter RabbitMqTransporter) PushEventCache(t string, data []byte) error {
	return nil
}
