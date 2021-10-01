package services

import (
	"context"
	"fmt"
	"github.com/discordnova/nova/common/management"
)

type statusServer struct {
	management.UnimplementedManagementServiceServer
}

func CreateStatusServerService() management.ManagementServiceServer {
	return &statusServer{}
}

func (s *statusServer) GetGlobalClusterStatus(context context.Context, _ *management.Empty) (*management.GlobalClusterStatusResponse, error) {
	return nil, fmt.Errorf("not implemented")
}
func (s *statusServer) GetClusterStatus(context context.Context, request *management.ClusterStatusRequest) (*management.ClusterStatusResponse, error) {
	return nil, fmt.Errorf("not implemented")
}

func (s *statusServer) GetShardStatus(context context.Context, request *management.ShardStatusRequest) (*management.ShardStatusResponse, error) {
	return nil, fmt.Errorf("not implemented")
}
