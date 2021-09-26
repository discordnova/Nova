package cmd

import (
	"fmt"
	"github.com/go-git/go-git/v5/config"
	"github.com/rs/zerolog/log"
	"io/fs"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"

	"github.com/go-git/go-git/v5"
	"github.com/spf13/cobra"
)

var (
	InitializeCommand         = createInitCommand()
	Flavour           *string = nil
	Name              *string = nil
)

func createInitCommand() *cobra.Command {
	command := cobra.Command{
		Use:   "init [path]",
		Short: "Initialize a new nova based project",
		Run:   initNovaRepo,
	}

	Flavour = command.Flags().String("flavour", "javascript", "the flavour of template to use")
	Name = command.Flags().String("name", "", "the name of the project")

	return &command
}

func determineTemplate() string {
	if strings.HasPrefix(*Flavour, "http") || strings.HasPrefix(*Flavour, "ssh") {
		return *Flavour
	} else {
		return fmt.Sprintf("https://github.com/discordnova/template-%s.git", *Flavour)
	}
}

func initNovaRepo(cmd *cobra.Command, args []string) {

	url := determineTemplate()
	if len(args) == 0 {
		log.Error().Msg("A path must be specified")
		os.Exit(1)
	}

	path := strings.Join(args, " ")
	name := path

	// if the user specified a name different from the folder name
	if *Name != "" {
		name = *Name
	}

	cw, err := os.Getwd()
	if err != nil {
		return
	}
	// we get the absolute path of the folder
	path = filepath.Join(cw, path)
	log.Info().Msgf("Initializing a %s at %s using template %s", name, path, url)

	// clone the repo
	_, err = git.PlainClone(path, false, &git.CloneOptions{
		URL: url,
	})

	if err != nil {
		log.Err(err).Msg("Failed to initialize the repository")
		os.Exit(1)
	}

	log.Info().Msg("Successfully cloned the template")

	// replace all the instances of "%PROJECT%" with the project name
	err = filepath.WalkDir(path, func(path string, d fs.DirEntry, err error) error {
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

	if err != nil {
		log.Err(err).Msgf("Failed to bootstrap the project")
		// we try to remove the folder
		_ = os.Remove(path)
		os.Exit(1)
	}
	// we remove the git folder
	err = os.RemoveAll(filepath.Join(path, ".git"))

	repo, err := git.PlainInit(path, false)
	if err != nil {
		log.Err(err).Msgf("Failed to initialize the git repository")
		os.Exit(1)
	}

	err = repo.CreateBranch(&config.Branch{
		Name: "main",
	})
	if err != nil {
		log.Err(err).Msgf("Failed to create the main branch")
		os.Exit(1)
	}

	tree, err := repo.Worktree()
	if err != nil {
		log.Err(err).Msgf("Failed to get worktree")
		os.Exit(1)
	}
	_, err = tree.Add(".")
	if err != nil {
		log.Err(err).Msgf("Failed to index the files")
		os.Exit(1)
	}
	_, err = tree.Commit("first commit", &git.CommitOptions{})
	if err != nil {
		log.Err(err).Msgf("Failed to index the first commit")
		os.Exit(1)
	}

	log.Info().Msgf("Created a new repository at %", path)
}
