package main

import (
	"github.com/erkrnt/wireguard-manager/api"
	"github.com/sirupsen/logrus"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

// MigrateDB : migrate the database with latest schema changes
func MigrateDB(db *gorm.DB) error {
	interfaceErr := db.AutoMigrate(&api.WireGuardInterface{})

	if interfaceErr != nil {
		return interfaceErr
	}

	peerErr := db.AutoMigrate(&api.WireGuardPeer{})

	if peerErr != nil {
		return peerErr
	}

	userErr := db.AutoMigrate(&api.User{})

	if userErr != nil {
		return userErr
	}

	logrus.Print("Successfully migrated database schemas")

	return nil
}

// OpenDB : open wrapper function for database
func OpenDB() (*gorm.DB, error) {
	db, err := gorm.Open(sqlite.Open("data.db"), &gorm.Config{})

	if err != nil {
		return nil, err
	}

	logrus.Debug("Successfully opened database connection")

	return db, nil
}
