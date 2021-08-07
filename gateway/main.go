package main

import (
	"flag"

	"github.com/discordnova/nova/common"
	"github.com/discordnova/nova/gateway/lib/gateway"
	"github.com/discordnova/nova/gateway/lib/gateway/compression"
	"github.com/discordnova/nova/gateway/lib/gateway/transporters"
	"github.com/rs/zerolog/log"
)

var (
	settings = gateway.GatewayConnectorOptions{
		Token:                  flag.String("token", "", "the discord token for the websocket connection"),
		Restart:                flag.Bool("restart", true, "should the gateway be restarted if an error occurs"),
		GuildSubs:              flag.Bool("guild-subscriptions", true, "should the guild subscription gateway flag set to true"),
		SelfShard:              flag.Int("shard", 0, "the shard id of this instance"),
		TotalShard:             flag.Int("shard-count", 1, "the total amount of shard"),
		SessionUpdateFrequency: flag.Int("session-persist-frequence", 10, "the frequency of session persistence"),
	}

	compressor  = flag.String("compressor", "json-zlib", "the compressor used to connect")
	transporter = flag.String("transporter", "rabbitmq", "the compressor used to connect")
	monitoring  = flag.Int("prometheus-port", 9000, "is this flag is set, a prometheus metrics endpoint will be exposed")
	url         = flag.String("transporter-url", "", "the url needed for rabbitmq to function")
)

func validate(settings *gateway.GatewayConnectorOptions) {
	if *settings.SelfShard > *settings.TotalShard {
		log.Fatal().Msg("invalid config: the shard id must be inferior than the total shard value")
	} else if *settings.SessionUpdateFrequency == 0 {
		log.Fatal().Msg("invalid config: the session update frequency muse be greater than 0")
	} else if *settings.Token == "" {
		log.Fatal().Msg("invalid config: invalid token provided")
	} else if *settings.TotalShard == 0 {
		log.Fatal().Msg("invalid config: the total number of shards muse be greater than 0")
	}
}

func main() {
	flag.Parse()
	common.SetupLogger()

	if monitoring != nil {
		go common.CreatePrometheus(*monitoring)
		log.Debug().Msg("prometheus server called")
	}

	if *compressor == "json-zlib" {
		settings.Compressor = compression.NewJsonZlibCompressor()
	} else {
		log.Fatal().Msgf("unknown compressor specified: %s", *compressor)
	}

	if *transporter == "rabbitmq" {
		trns, err := transporters.NewRabbitMqTransporter(*url)
		if err != nil {
			log.Fatal().Msgf("failed to initialize the transporter: %s", err.Error())
		}
		settings.Transporter = trns
	}

	validate(&settings)

	gateway := gateway.NewGateway(settings)
	gateway.Start()
}
