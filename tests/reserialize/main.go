package main

import (
	"github.com/dragonfireclient/mt"
	"os"
)

func main() {
	toSrv := os.Args[1] == "ToSrvPkt"

	pkt, err := mt.DeserializePkt(os.Stdin, !toSrv)
	if err != nil {
		os.Stderr.WriteString(err.Error())
		os.Exit(1)
	}

	mt.SerializePkt(*pkt, os.Stdout, toSrv)
}
