package gateway

import (
	"encoding/json"
	"fmt"
	"os"
	"runtime"
	"time"

	"github.com/boz/go-throttle"
	gatewayTypes "github.com/discordnova/nova/common/discord/types/payloads/gateway"
	"github.com/discordnova/nova/common/discord/types/payloads/gateway/commands"
	"github.com/discordnova/nova/common/discord/types/payloads/gateway/events"
	"github.com/discordnova/nova/common/discord/types/structures"
	"github.com/discordnova/nova/common/discord/types/types"
	"github.com/discordnova/nova/common/gateway"
	"github.com/gorilla/websocket"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/rs/zerolog/log"
)

// connectionState is a struct representing a connection state
type connectionState struct {
	HeartbeatInterval uint16
	Latency           int64
}

var (
	messagesCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "nova_gateway_messages_processed",
		Help: "The total number of processed messages",
	})

	heartbeatCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "nova_gateway_heartbeat_sent",
		Help: "The total number of heartbeat sent",
	})

	latencyGauge = promauto.NewGauge(prometheus.GaugeOpts{
		Name: "nova_gateway_latency",
		Help: "The round trip latency of the gateway",
	})

	reconnectionsCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "nova_gateway_reconnections",
		Help: "the number of reconnections of the gateway",
	})

	eventsCounter = promauto.NewCounter(prometheus.CounterOpts{
		Name: "nova_gateway_events",
		Help: "The various events received by Nova.",
	})
)

// GatewayConnector represents a connection to the discord gateway for a shard
type GatewayConnector struct {
	// Public State
	SessionState GatewayConnectorOptionsResume // The state of the session

	// Private state
	connectionState connectionState         // The internal state of the gateway connection.
	options         GatewayConnectorOptions // The connection options.
	connection      *websocket.Conn         // The current websocket connection.
	heartbeat       chan struct{}           // Channel for reacting to heartbeat acks
	terminate       chan string             // Called when a gateway disconnect is requested
	updateThrottle  throttle.ThrottleDriver
}

// NewGateway creates a connector instance based on the given options.
func NewGateway(options GatewayConnectorOptions) *GatewayConnector {
	return &GatewayConnector{
		options:      options,
		SessionState: options.ResumeSession,
	}
}

// Start is used to start or reset a connection to the gateway.
func (discord *GatewayConnector) Start() {
	shouldStart := true
	for shouldStart {
		reconnectionsCounter.Inc()
		discord.connectionState = connectionState{}
		_ = discord.start()
		err := discord.options.Compressor.Reset()
		if err != nil {
			log.Fatal().Msgf("failed to reset the compressor")
		}
		shouldStart = *discord.options.Restart
		if shouldStart {
			log.Info().Msg("waiting 10s before gateway reconnection")
			time.Sleep(time.Second * 10)
		}
	}
}

// start is the internal routine for starting the gateway
func (discord *GatewayConnector) start() error {
	// we throttle the update function to limit the amount of session state
	// presisted to the session persistence interface
	discord.updateThrottle = throttle.ThrottleFunc(time.Second*5, false, func() {
		if discord.options.OnSessionStateUpdated != nil {
			_ = discord.options.OnSessionStateUpdated(discord.SessionState)
		}
	})

	// initialize the message channels
	discord.heartbeat = make(chan struct{})
	discord.terminate = make(chan string)

	// since a Compressor is given to the gateway when created, we use the Connector to get
	// the compression and encoding options.
	comOptions := discord.options.Compressor.GetConnectionOptions()
	websocketURL := fmt.Sprintf("wss://gateway.discord.gg/?v=%d&encoding=%s&compress=%s", 6, comOptions.Encoding, comOptions.TransportCompression)

	log.Info().Msgf("connecting to the gateway at url %s", websocketURL)
	// we start the connection to discord.
	connection, _, err := websocket.DefaultDialer.Dial(websocketURL, nil)
	if err != nil {
		log.Err(err).Msgf("an error occurred while connecting to the gateway")
		return err
	}
	discord.connection = connection
	defer discord.connection.Close()

	// start listening to messages on the socket.
	go discord.listen()

	msg := <-discord.terminate
	log.Info().Msgf("terminating the gateway: %s", msg)

	return nil
}

// ticker starts the loop for the heartbeat checks
func (discord *GatewayConnector) ticker(interval int) {
	// sends a message to heartbeat.C every time we need to send a heartbeat
	heartbeat := time.NewTicker(time.Duration(interval) * time.Millisecond)
	// stores if the last heartbeat succeeded
	doneLastAck := true

	// executes the given code when heartbeat.C is triggered
	for range heartbeat.C {
		// if the server did not send the last heartbeat
		if !doneLastAck {
			// we need to terminate the connection
			discord.terminate <- "server missed an ack and must be disconnected"
			return
		}

		log.Debug().Msg("Sending a heartbeat.")

		index, _ := json.Marshal(discord.SessionState.Index)
		err := discord.connection.WriteJSON(gatewayTypes.Payload{
			Op: types.GatewayOpCodeHeartbeat,
			D:  index,
		})

		if err != nil {
			discord.terminate <- fmt.Sprintf("failed to send a heartbeat: %s", err.Error())
			return
		}

		heartbeatCounter.Inc()
		// wait for the ack asynchronously
		go func() {
			start := time.Now()
			doneLastAck = false
			<-discord.heartbeat
			doneLastAck = true

			discord.connectionState.Latency = time.Since(start).Milliseconds()
			latencyGauge.Set(float64(discord.connectionState.Latency))

			log.Info().Msgf("heartbeat completed, latency: %dms", discord.connectionState.Latency)
		}()

	}
}

// listen listens to the messages on the gateway
func (discord *GatewayConnector) listen() {
	for {
		_, message, err := discord.connection.ReadMessage()

		if err != nil {
			discord.terminate <- fmt.Sprintf("the connection was closed by the gateway: %s", err.Error())
			return
		}

		messagesCounter.Inc()
		data, err := discord.options.Compressor.DecodeMessage(message)

		if err != nil || data == nil {
			log.Print(err.Error())
			continue
		}

		if data.S != 0 {
			discord.SessionState.Index = data.S
			discord.updateState(data.S, "")
		}

		discord.handleMessage(data)
	}
}

func (discord *GatewayConnector) updateState(newIndex int64, sessionId string) {
	discord.SessionState.Index = newIndex
	if sessionId != "" {
		discord.SessionState.Session = sessionId
	}
	discord.updateThrottle.Trigger()
}

func (discord *GatewayConnector) handleMessage(message *gatewayTypes.Payload) {
	switch message.Op {
	// call the startup function
	case types.GatewayOpCodeHello:
		discord.hello(message)
	// notify the heartbeat goroutine that a heartbeat ack was received
	case types.GatewayOpCodeHeartbeatACK:
		discord.heartbeat <- struct{}{}
	// handles a dispatch from the gateway
	case types.GatewayOpCodeDispatch:
		discord.dispatch(message)
	// when the session resume fails
	case types.GatewayOpCodeInvalidSession:
		log.Print("failed to resume the session, reconnecting")
		discord.updateState(0, "")
		discord.doLogin()
	// when the gateway requests a reconnect
	case types.GatewayOpCodeReconnect:
		log.Print("the gateway requested a reconnect")
		if string(message.D) != "true" {
			// we may delete the session state
			discord.SessionState.Index = 0
			discord.updateState(0, "")
		}
		discord.terminate <- "the gateway requested a reconnect"
	}
}

func (discord *GatewayConnector) doLogin() {
	var payload gatewayTypes.Payload
	// if we do not have to resume a session
	if discord.SessionState.Session == "" {
		log.Info().Msg("using identify for authentification")
		data, err := json.Marshal(commands.GatewayCommandIdentifyPayload{
			Token: *discord.options.Token,
			Properties: structures.IdentifyConnectionProperties{
				OS:      runtime.GOOS,
				Device:  "Nova Discord Client",
				Browser: "Nova Discord Client",
			},
			Compress:       true,
			LargeThreshold: 1000,
			Shard: []int{
				*discord.options.SelfShard,
				*discord.options.TotalShard,
			},
			Presence:           commands.GatewayCommandUpdateStatusPayload{},
			GuildSubscriptions: *discord.options.GuildSubs,
			Intents:            discord.options.Intend,
		})

		if err != nil {
			return
		}

		payload = gatewayTypes.Payload{
			Op: types.GatewayOpCodeIdentify,
			D:  data,
		}
	} else {
		log.Info().Msg("resuming session")
		data, err := json.Marshal(commands.GatewayCommandResumePayload{
			Token:     *discord.options.Token,
			SessionID: discord.SessionState.Session,
			Seq:       discord.SessionState.Index,
		})

		if err != nil {
			return
		}
		payload = gatewayTypes.Payload{
			Op: types.GatewayOpCodeResume,
			D:  data,
		}
	}

	err := discord.connection.WriteJSON(payload)
	if err != nil {
		log.Err(err).Msgf("failed send the identify payload")
	}
}

func (discord *GatewayConnector) hello(message *gatewayTypes.Payload) {

	data := &events.GatewayEventHelloPayload{}
	err := json.Unmarshal(message.D, &data)
	if err != nil {
		discord.terminate <- fmt.Sprintf("invalid payload: %s", err.Error())
	}

	// start the heartbeat goroutine
	log.Debug().Msgf("hello recevied, heartbeating every %d ms", data.HeartbeatInterval)
	go discord.ticker(data.HeartbeatInterval)

	// login
	discord.doLogin()
}

type NovaMessage struct {
	Data    json.RawMessage `json:"data"`
	Tracing struct {
		NodeName string `json:"node_name"`
	} `json:"tracing"`
}

func (discord *GatewayConnector) dispatch(message *gatewayTypes.Payload) {
	// since this is juste a event gateway, we do not care about the content of the events
	// except the ready, resumed, reconnect event we use to update the session_id, the other events are forwarded to the transporter
	if message.T == "READY" {
		event := events.GatewayEventReadyPayload{}
		err := json.Unmarshal(message.D, &event)

		log.Info().Msgf("logged in as %s", event.User.Username)

		if err != nil {
			discord.terminate <- "invalid ready event"
			return
		}

		discord.updateState(discord.SessionState.Index, event.SessionID)
		return
	}

	newName := gateway.EventNames[message.T]

	if newName == "" {
		log.Error().Msgf("unknown event name: %s", newName)
		return
	}

	name, err := os.Hostname()

	if err != nil {
		log.Err(err).Msgf("failed to get the hostname")
		return
	}

	data, err := json.Marshal(NovaMessage{
		Data: message.D,
		Tracing: struct {
			NodeName string `json:"node_name"`
		}{
			NodeName: name,
		},
	})

	if err != nil {
		log.Err(err).Msg("failed to serialize the outgoing nova message")
	}

	discord.options.Transporter.PushChannel() <- gateway.PushData{
		Data: data,
		Name: newName,
	}

	if err != nil {
		log.Err(err).Msg("failed to send the event to the nova event broker")
	}
}
