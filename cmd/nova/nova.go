package main

import (
	"os"
	"os/signal"
	"syscall"

	allinone "github.com/discordnova/nova/internal/pkg/all-in-one"
)

func main() {
	allInOne, err := allinone.NewAllInOne()
	if err != nil {
		panic(err)
	}
	err = allInOne.Start()
	if err != nil {
		panic(err)
	}
	// Wait for a SIGINT
	c := make(chan os.Signal, 1)
	signal.Notify(c,
		syscall.SIGHUP,
		syscall.SIGINT,
		syscall.SIGTERM,
		syscall.SIGQUIT)
	<-c

	allInOne.Stop()

	println("Arret de nova all in one")
}
