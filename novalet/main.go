package main

import (
	websocketLib "github.com/discordnova/nova/gateway/lib/gateway"
	"github.com/discordnova/nova/novalet/lib"
)

func main() {
	go lib.NewNatsStandalone()
	transporter, _ := lib.NewInternalTransporter()
	websocket := websocketLib.NewGateway(websocketLib.GatewayConnectorOptions{
		Transporter: transporter,
	})

	go websocket.Start()
}
