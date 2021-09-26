package lib

import (
	"google.golang.org/grpc"

	"github.com/discordnova/nova/common/management"
)

// NewConn creates a connection to the manager
func NewConn(host string) (*management.ManagementServiceClient, error) {
	lis, err := grpc.Dial(host, grpc.WithInsecure())
	if err != nil {
		return nil, err
	}

	conn := management.NewManagementServiceClient(lis)

	return &conn, nil
}
