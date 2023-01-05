package main

/*
#cgo LDFLAGS: -L./build -lall -lz -lm
#include "./build/all.h"
*/
import "C"

import (
	"fmt"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/Jeffail/gabs"
	"github.com/alicebob/miniredis/v2"

	server "github.com/nats-io/nats-server/v2/server"
)

func main() {
	// Intialise les logs de la librarie Rust
	C.init_logs()
	// Charge la configuration
	str := C.GoString(C.load_config())

	// Démarre une instance MiniRedis
	mr := miniredis.NewMiniRedis()
	err := mr.Start()

	if err != nil {
		panic(err)
	}

	// Démarre un serveur Nats
	opts := &server.Options{}
	opts.Host = "0.0.0.0"
	ns, err := server.NewServer(opts)

	if err != nil {
		panic(err)
	}

	go ns.Start()

	if !ns.ReadyForConnections(4 * time.Second) {
		panic("not ready for connection")
	}

	// Edite le json de configuration donné
	// Et injecte la configuration des servers Nats et MiniRedis
	json, _ := gabs.ParseJSON([]byte(str))
	json.Set(fmt.Sprintf("redis://%s", mr.Addr()), "redis", "url")
	json.Set("localhost", "nats", "host")
	json.Set(1, "webhook", "discord", "client_id")

	// Démarre une instance de nova
	instance := C.start_instance(C.CString(json.String()))

	// Wait for a SIGINT
	c := make(chan os.Signal, 1)
	signal.Notify(c,
		syscall.SIGHUP,
		syscall.SIGINT,
		syscall.SIGTERM,
		syscall.SIGQUIT)
	<-c

	println("Arret de nova all in one")
	C.stop_instance(instance)
}
