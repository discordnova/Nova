package cmd

import (
	"fmt"
	"io/fs"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"

	"github.com/TwinProduction/go-color"
	"github.com/go-git/go-git/v5"
	"github.com/spf13/cobra"
)

var (
	InitializeCommand = &cobra.Command{
		Use:   "init",
		Short: "Initialize a new nova based project",
		Run:   initNovaRepo,
	}
)

func initNovaRepo(cmd *cobra.Command, args []string) {
	url := "https://github.com/libgit2/git2go.git"
	path := ""
	name := "test"

	if name == "" {
		fmt.Print(
			color.Ize(color.Red, "A name must be specified"),
		)
		return
	}

	if path == "" {
		path = fmt.Sprintf("./%s", name)
	}
	cw, err := os.Getwd()
	if err != nil {
		return
	}
	path = filepath.Join(cw, path)

	fmt.Println(
		color.Ize(color.Green, fmt.Sprintf("Initializing a new nova project at %s", path)),
	)
	fmt.Println(
		color.Ize(color.Gray, fmt.Sprintf("Using the %s template", url)),
	)

	// clone the repo
	_, err = git.PlainClone(path, false, &git.CloneOptions{
		URL:      url,
		Progress: os.Stdout,
	})

	if err != nil {
		fmt.Println(
			color.Ize(color.Red, fmt.Sprintf("Failed to initialize the repository: %s", err.Error())),
		)
		return
	}

	fmt.Println(
		color.Ize(color.Green, "Cloned the repository..."),
	)

	filepath.WalkDir(path, func(path string, d fs.DirEntry, err error) error {
		if d.IsDir() || err != nil {
			return nil
		}

		read, err := ioutil.ReadFile(path)
		if err != nil {

		}
		content := strings.ReplaceAll(string(read), "%PROJECT%", name)

		err = ioutil.WriteFile(path, []byte(content), 0)

		if err != nil {
			return err
		}
		return nil
	})

	err = os.RemoveAll(filepath.Join(path, ".git"))
}
