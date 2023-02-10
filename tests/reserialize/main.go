package main

import (
	"github.com/dragonfireclient/mt"
	"os"
)

// WIP: test against the Go mt package
func main() {
	pkt, err := mt.DeserializePkt(os.Stdin, false)
	if err != nil {
		panic(err)
	}

	mt.SerializePkt(*pkt, os.Stdout, false)
}
