package internal

import (
	"gorm.io/gorm"
)

// Service : service instance
type Service struct {
	DB *gorm.DB
}

// NewService : creates a new "wireguard-manager" service instance
func NewService() (*Service, error) {
	db, err := OpenDB()

	if err != nil {
		return nil, err
	}

	migrateErr := MigrateDB(db)

	if migrateErr != nil {
		return nil, migrateErr
	}

	service := Service{
		DB: db,
	}

	return &service, nil
}
