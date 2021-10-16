package cmd

import (
	"context"
	"errors"
	"github.com/discordnova/nova/common/management"
	"github.com/discordnova/nova/novactl/lib"
	"github.com/olekukonko/tablewriter"
	"github.com/rs/zerolog/log"
	"github.com/spf13/cobra"
	"os"
	"strconv"
	"time"
)

var ClusterCommand = createClusterCommand()

var (
	ServerUrl *string = nil
	Ctx               = context.Background()
)

func createClusterCommand() *cobra.Command {
	command := &cobra.Command{
		Use:              "cluster",
		Short:            "Commands to interact with the nova cluster",
		Aliases:          []string{"c"},
		TraverseChildren: true,
	}
	// shard sub command
	shard := cobra.Command{
		Use:   "shard [shard]",
		Short: "Returns information about a specific shard",
		Run:   shardCommand,
		Args: func(cmd *cobra.Command, args []string) error {
			if len(args) != 1 {
				return errors.New("one shard id must be specified")
			} else {
				if _, err := strconv.Atoi(args[0]); err != nil {
					return errors.New("the shard id must be a string")
				}
			}
			return nil
		},
		TraverseChildren: true,
	}
	// info sub command
	info := cobra.Command{
		Use:              "info",
		Short:            "Gets the status of the cluster",
		Aliases:          []string{"i"},
		Run:              infoCommand,
		TraverseChildren: true,
	}

	ServerUrl = command.Flags().StringP("server", "s", "localhost:8053", "")

	command.AddCommand(&shard)
	command.AddCommand(&info)

	return command
}

func shardCommand(command *cobra.Command, args []string) {
	id, err := strconv.ParseInt(args[0], 10, 64)
	if err != nil {
		log.Err(err).Msg("Failed to parse the shard id")
		os.Exit(1)
	}

	log.Info().Msgf("Starting connection with server %s", *ServerUrl)
	conn, err := lib.NewConn(*ServerUrl)

	if err != nil {
		log.Err(err).Msg("Failed to connect to the manager")
		return
	}
	ctx, _ := context.WithTimeout(Ctx, time.Second*10)

	manager := *conn
	data, err := manager.GetShardStatus(ctx, &management.ShardStatusRequest{
		Identifier: id,
	})

	if err != nil {
		log.Err(err).Msg("Failed to get the status of the shard")
		return
	}

	table := tablewriter.NewWriter(os.Stdout)
	table.SetHeader([]string{"Status", "Id", "Cluster", "Latency"})
	table.Append([]string{string(data.Status), string(data.Identifier), data.Cluster, string(data.Latency)})
	table.Render()
}

func infoCommand(command *cobra.Command, args []string) {}
