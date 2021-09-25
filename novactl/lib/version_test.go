package lib_test

import (
	"testing"

	"github.com/discordnova/nova/novactl/lib"
)

func TestVersion(t *testing.T) {
	if lib.VERSION != "0.0.1" {
		t.Fatalf("Version number do not match %s", lib.VERSION)
	}
}
