package lib

import (
	"github.com/discordnova/nova/common/management"
	"github.com/discordnova/nova/manager/lib/services"
	"google.golang.org/grpc"
	"log"
	"net"
)

func StartGrpcServices(config *Config) {
	lis, err := net.Listen("tcp", config.Grpc.Server)

	if err != nil {
		log.Fatalf("failed to start the grpc services: %v", err)
	}
	var opts []grpc.ServerOption

	grpcServer := grpc.NewServer(opts...)

	management.RegisterManagementServiceServer(grpcServer, services.CreateStatusServerService())

	err = grpcServer.Serve(lis)
	if err != nil {
		log.Fatalf("failed to start the grpc server: %v", err)
	}
}
