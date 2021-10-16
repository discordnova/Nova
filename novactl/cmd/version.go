package cmd

import (
	"fmt"

	"github.com/discordnova/nova/common"
	"github.com/spf13/cobra"
)

var (
	VersionCommand = &cobra.Command{
		Use:   "version",
		Short: "Returns the version of the CLI",
		Run:   version,
	}
)

func version(cmd *cobra.Command, args []string) {
	fmt.Println(fmt.Sprintf("Nova version: %s", common.VERSION))
}
