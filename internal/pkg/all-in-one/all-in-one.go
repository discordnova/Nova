package allinone

/*
#cgo LDFLAGS: -L../../../build/lib -lall_in_one -lz -lm
#include "./all_in_one.h"
#include "./error_handler.h"
*/
import "C"
import (
	"fmt"
	"time"

	"github.com/Jeffail/gabs"
	"github.com/alicebob/miniredis/v2"
	"github.com/nats-io/nats-server/v2/server"
)

type AllInOne struct {
	redis    *miniredis.Miniredis
	nats     *server.Server
	instance *C.AllInOneInstance
}

func NewAllInOne() (*AllInOne, error) {
	redis := miniredis.NewMiniRedis()
	nats, err := server.NewServer(&server.Options{})

	if err != nil {
		return nil, err
	}

	return &AllInOne{
		redis: redis,
		nats:  nats,
	}, nil
}

func (s *AllInOne) Start() error {
	err := s.redis.Start()
	if err != nil {
		return err
	}

	go s.nats.Start()

	if !s.nats.ReadyForConnections(5 * time.Second) {
		return fmt.Errorf("nats server didn't start after 5 seconds, please check if there is another service listening on the same port as nats")
	}

	handler := C.ErrorHandler(C.allInOneErrorHandler)
	// Set the error handler
	C.set_error_handler(handler)
	config := C.GoString(C.load_config())

	json, _ := gabs.ParseJSON([]byte(config))
	json.Set(fmt.Sprintf("redis://%s", s.redis.Addr()), "redis", "url")
	json.Set("localhost", "nats", "host")
	json.Set(1, "webhook", "discord", "client_id")

	a := ""
	a += ("Starting nova All-in-one!\n")
	a += fmt.Sprintf(" * Rest proxy running on         : http://%s\n", json.Path("rest.server.listening_adress").Data().(string))
	a += fmt.Sprintf(" * Webhook server running on     : http://%s\n", json.Path("webhook.server.listening_adress").Data().(string))
	a += fmt.Sprintf(" * Ratelimiter server running on : grpc://%s\n", json.Path("ratelimiter.server.listening_adress").Data().(string))
	a += (" * The gateway server should be running\n")
	a += (" * The cache server should be running\n")
	a += (" * Servers\n")
	a += fmt.Sprintf("    * Running MiniREDIS on %s\n", s.redis.Addr())
	a += fmt.Sprintf("    * Running NATS on %s\n", s.nats.ClientURL())
	s.instance = C.create_instance(C.CString(json.String()))

	print(a)

	return nil
}

func (s *AllInOne) Stop() {
	C.stop_instance(s.instance)
}
