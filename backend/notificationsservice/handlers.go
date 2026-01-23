package main

import (
	"encoding/json"
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

// CreateNotification crea una nueva notificación
func CreateNotification(c *gin.Context) {
	var req CreateNotificationRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Crear notificación
	notification := &Notification{
		ID:        uuid.New().String(),
		Type:      req.Type,
		Recipient: req.Recipient,
		Subject:   req.Subject,
		Message:   req.Message,
		Status:    "pending",
		OrderID:   req.OrderID,
		Metadata:  req.Metadata,
		CreatedAt: time.Now(),
	}

	// Guardar en base de datos
	SaveNotification(notification)

	// Intentar enviar email
	go func(notif *Notification) {
		err := SendEmail(notif.Recipient, notif.Subject, notif.Message)
		now := time.Now()

		if err != nil {
			notif.Status = "failed"
			notif.Error = err.Error()
			log.Printf("  Failed to send notification %s: %v", notif.ID, err)
		} else {
			notif.Status = "sent"
			notif.SentAt = &now
			log.Printf("  Notification %s sent successfully", notif.ID)
		}

		UpdateNotification(notif)
	}(notification)

	c.JSON(http.StatusCreated, notification)
}

// GetNotifications obtiene todas las notificaciones
func GetNotifications(c *gin.Context) {
	notifications := GetAllNotifications()
	c.JSON(http.StatusOK, gin.H{
		"total":         len(notifications),
		"notifications": notifications,
	})
}

// GetNotificationByID obtiene una notificación por ID
func GetNotificationByID(c *gin.Context) {
	id := c.Param("id")

	notification, exists := GetNotification(id)
	if !exists {
		c.JSON(http.StatusNotFound, gin.H{"error": "Notification not found"})
		return
	}

	c.JSON(http.StatusOK, notification)
}

// NotifyOrderCreated notifica la creación de un pedido
func NotifyOrderCreated(c *gin.Context) {
	var req OrderCreatedRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Generar email HTML
	emailBody := GenerateOrderCreatedEmail(req)

	// Crear metadata como JSON
	metadata, _ := json.Marshal(map[string]interface{}{
		"order_id":      req.OrderID,
		"customer_name": req.CustomerName,
		"total_amount":  req.TotalAmount,
		"items_count":   len(req.Items),
	})

	// Crear notificación
	notification := &Notification{
		ID:        uuid.New().String(),
		Type:      "order_created",
		Recipient: req.CustomerEmail,
		Subject:   "¡Tu pedido ha sido confirmado! #" + req.OrderID,
		Message:   emailBody,
		Status:    "pending",
		OrderID:   req.OrderID,
		Metadata:  string(metadata),
		CreatedAt: time.Now(),
	}

	// Guardar en base de datos
	SaveNotification(notification)

	// Enviar email de forma asíncrona
	go func(notif *Notification) {
		err := SendEmail(notif.Recipient, notif.Subject, notif.Message)
		now := time.Now()

		if err != nil {
			notif.Status = "failed"
			notif.Error = err.Error()
			log.Printf("  Failed to send order notification %s: %v", notif.ID, err)
		} else {
			notif.Status = "sent"
			notif.SentAt = &now
			log.Printf("  Order notification %s sent successfully to %s", notif.ID, notif.Recipient)
		}

		UpdateNotification(notif)
	}(notification)

	c.JSON(http.StatusCreated, gin.H{
		"message":         "Order notification created and being sent",
		"notification_id": notification.ID,
		"order_id":        req.OrderID,
	})
}

// NotifyStatusChange notifica un cambio de estado
func NotifyStatusChange(c *gin.Context) {
	var req StatusChangeRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Generar email HTML
	emailBody := GenerateStatusChangeEmail(req)

	// Crear metadata como JSON
	metadata, _ := json.Marshal(map[string]interface{}{
		"order_id":   req.OrderID,
		"old_status": req.OldStatus,
		"new_status": req.NewStatus,
	})

	// Crear notificación
	notification := &Notification{
		ID:        uuid.New().String(),
		Type:      "status_change",
		Recipient: req.CustomerEmail,
		Subject:   "Actualización de tu pedido #" + req.OrderID,
		Message:   emailBody,
		Status:    "pending",
		OrderID:   req.OrderID,
		Metadata:  string(metadata),
		CreatedAt: time.Now(),
	}

	// Guardar en base de datos
	SaveNotification(notification)

	// Enviar email de forma asíncrona
	go func(notif *Notification) {
		err := SendEmail(notif.Recipient, notif.Subject, notif.Message)
		now := time.Now()

		if err != nil {
			notif.Status = "failed"
			notif.Error = err.Error()
			log.Printf("  Failed to send status change notification %s: %v", notif.ID, err)
		} else {
			notif.Status = "sent"
			notif.SentAt = &now
			log.Printf("  Status change notification %s sent successfully to %s", notif.ID, notif.Recipient)
		}

		UpdateNotification(notif)
	}(notification)

	c.JSON(http.StatusCreated, gin.H{
		"message":         "Status change notification created and being sent",
		"notification_id": notification.ID,
		"order_id":        req.OrderID,
		"new_status":      req.NewStatus,
	})
}

// NotifyPaymentRejected notifica un pago rechazado
func NotifyPaymentRejected(c *gin.Context) {
	var req PaymentRejectedRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Generar email HTML
	emailBody := GeneratePaymentRejectedEmail(req)

	// Crear metadata como JSON
	metadata, _ := json.Marshal(map[string]interface{}{
		"order_id":         req.OrderID,
		"amount":           req.Amount,
		"transaction_id":   req.TransactionID,
		"rejection_reason": req.RejectionReason,
	})

	// Crear notificación
	notification := &Notification{
		ID:        uuid.New().String(),
		Type:      "payment_rejected",
		Recipient: req.CustomerEmail,
		Subject:   "⚠️ Pago Rechazado - Pedido #" + req.OrderID,
		Message:   emailBody,
		Status:    "pending",
		OrderID:   req.OrderID,
		Metadata:  string(metadata),
		CreatedAt: time.Now(),
	}

	// Guardar en base de datos
	SaveNotification(notification)

	// Enviar email de forma asíncrona
	go func(notif *Notification) {
		err := SendEmail(notif.Recipient, notif.Subject, notif.Message)
		now := time.Now()

		if err != nil {
			notif.Status = "failed"
			notif.Error = err.Error()
			log.Printf("  Failed to send payment rejected notification %s: %v", notif.ID, err)
		} else {
			notif.Status = "sent"
			notif.SentAt = &now
			log.Printf("  Payment rejected notification %s sent successfully to %s", notif.ID, notif.Recipient)
		}

		UpdateNotification(notif)
	}(notification)

	c.JSON(http.StatusCreated, gin.H{
		"message":         "Payment rejected notification created and being sent",
		"notification_id": notification.ID,
		"order_id":        req.OrderID,
	})
}
