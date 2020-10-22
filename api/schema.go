package api

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

// BaseSchema : base struct for db structs
type BaseSchema struct {
	ID        uuid.UUID `gorm:"primarykey"`
	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt gorm.DeletedAt `gorm:"index"`
}

// WireGuardInterface : a WireGuard interface
type WireGuardInterface struct {
	BaseSchema
	Address    string
	ListenPort int
	PostUp     string
	PostDown   string
	SaveConfig bool
}

// WireGuardPeer : a WireGuard peer
type WireGuardPeer struct {
	BaseSchema
	Address    string
	PrivateKey string
	PublicKey  string
	UserID     string
}

// User : a user
type User struct {
	BaseSchema
	Name string
}

// BeforeCreate : custom hook for adding UUID's and other things to structs before creating
func (b *BaseSchema) BeforeCreate(tx *gorm.DB) error {
	b.ID = uuid.New()
	return nil
}

// TableName : sets the WireGuardInterface table name
func (wgi *WireGuardInterface) TableName() string {
	return "wireguard_interface"
}

// TableName : sets the WireGuardPeer table name
func (wgp *WireGuardPeer) TableName() string {
	return "wireguard_peer"
}
