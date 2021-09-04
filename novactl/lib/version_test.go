package lib_test

import (
	"github.com/discordnova/nova/novactl/lib"
	"testing"
)

func TestVersion(t *testing.T) {
    if lib.VERSION != "0.0.1" {
		t.Fatalf("Version number do not match %s", lib.VERSION)
	}
}