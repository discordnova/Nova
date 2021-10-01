package main

import (
	"github.com/buraksezer/consistent"
	"github.com/discordnova/nova/manager/lib"
	"github.com/discordnova/nova/manager/lib/scheduler"
	"log"
)

type myMember string

func (m myMember) String() string {
	return string(m)
}

func main () {
	scheduler.CreateScheduler(scheduler.State{
		Members:        []consistent.Member{myMember("cluster1"), myMember("cluster2"), myMember("cluster3"), myMember("cluster4")},
		CurrentSize:    2,
		MaxConcurrency: 5,
	}).Scale(10)
	log.Printf("starting grpc")
	lib.StartGrpcServices(&lib.Config{
		Grpc: lib.GrpcConfig{Server: "0.0.0.0:8053"},
	})
}