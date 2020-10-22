package main

import (
	_ "github.com/mattn/go-sqlite3"
	"github.com/sirupsen/logrus"
)

func main() {
	logrus.SetLevel(logrus.DebugLevel)

	_, err := NewService()

	if err != nil {
		logrus.Fatal(err)
	}
}
