package main

import (
	"github.com/erkrnt/wireguard-manager/internal"
	_ "github.com/mattn/go-sqlite3"
	"github.com/sirupsen/logrus"
)

func main() {
	logrus.SetLevel(logrus.DebugLevel)

	_, err := internal.NewService()

	if err != nil {
		logrus.Fatal(err)
	}
}
