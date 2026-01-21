package main

import (
	"sync"
)

// Base de datos en memoria para notificaciones
var (
	notifications = make(map[string]*Notification)
	notifMutex    = &sync.RWMutex{}
)

// InitDB inicializa la base de datos en memoria
func InitDB() {
	notifications = make(map[string]*Notification)
}

// SaveNotification guarda una notificación en la base de datos
func SaveNotification(notification *Notification) {
	notifMutex.Lock()
	defer notifMutex.Unlock()
	notifications[notification.ID] = notification
}

// GetNotification obtiene una notificación por ID
func GetNotification(id string) (*Notification, bool) {
	notifMutex.RLock()
	defer notifMutex.RUnlock()
	notif, exists := notifications[id]
	return notif, exists
}

// GetAllNotifications obtiene todas las notificaciones
func GetAllNotifications() []*Notification {
	notifMutex.RLock()
	defer notifMutex.RUnlock()
	
	result := make([]*Notification, 0, len(notifications))
	for _, notif := range notifications {
		result = append(result, notif)
	}
	return result
}

// UpdateNotification actualiza una notificación existente
func UpdateNotification(notification *Notification) bool {
	notifMutex.Lock()
	defer notifMutex.Unlock()
	
	if _, exists := notifications[notification.ID]; exists {
		notifications[notification.ID] = notification
		return true
	}
	return false
}
