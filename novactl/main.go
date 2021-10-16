package main

import (
	"github.com/discordnova/nova/novactl/cmd"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/spf13/cobra"
	"os"
)

func main() {
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr})

	rootCommand := &cobra.Command{
		Use: "novactl",
		Short: "A tool to interact with a nova cluster",
		TraverseChildren: true,
	}
	rootCommand.AddCommand(cmd.VersionCommand)
	rootCommand.AddCommand(cmd.InitializeCommand)
	rootCommand.AddCommand(cmd.ClusterCommand)
	_ = rootCommand.Execute()
}
