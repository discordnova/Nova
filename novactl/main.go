package main

import (
	"github.com/discordnova/nova/novactl/cmd"
	"github.com/spf13/cobra"
)

func main() {
	rootCommand := &cobra.Command{Use: "app"}
	rootCommand.AddCommand(cmd.VersionCommand)
	rootCommand.AddCommand(cmd.InitializeCommand)
	rootCommand.Execute()
}
