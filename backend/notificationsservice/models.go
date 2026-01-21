package main

import (
	"time"
)

// Notification representa una notificación en el sistema
type Notification struct {
	ID        string    `json:"id"`
	Type      string    `json:"type"` // "order_created", "status_change", "general"
	Recipient string    `json:"recipient"` // email del destinatario
	Subject   string    `json:"subject"`
	Message   string    `json:"message"`
	Status    string    `json:"status"` // "pending", "sent", "failed"
	OrderID   string    `json:"order_id,omitempty"`
	Metadata  string    `json:"metadata,omitempty"` // JSON string con datos adicionales
	CreatedAt time.Time `json:"created_at"`
	SentAt    *time.Time `json:"sent_at,omitempty"`
	Error     string    `json:"error,omitempty"`
}

// CreateNotificationRequest representa la solicitud para crear una notificación
type CreateNotificationRequest struct {
	Type      string `json:"type" binding:"required"`
	Recipient string `json:"recipient" binding:"required,email"`
	Subject   string `json:"subject" binding:"required"`
	Message   string `json:"message" binding:"required"`
	OrderID   string `json:"order_id,omitempty"`
	Metadata  string `json:"metadata,omitempty"`
}

// OrderCreatedRequest representa la notificación de creación de pedido
type OrderCreatedRequest struct {
	OrderID      string  `json:"order_id" binding:"required"`
	CustomerEmail string `json:"customer_email" binding:"required,email"`
	CustomerName  string `json:"customer_name" binding:"required"`
	TotalAmount   float64 `json:"total_amount" binding:"required"`
	Items         []OrderItem `json:"items"`
}

// OrderItem representa un item del pedido
type OrderItem struct {
	ProductName string  `json:"product_name"`
	Quantity    int     `json:"quantity"`
	Price       float64 `json:"price"`
}

// StatusChangeRequest representa la notificación de cambio de estado
type StatusChangeRequest struct {
	OrderID       string `json:"order_id" binding:"required"`
	CustomerEmail string `json:"customer_email" binding:"required,email"`
	OldStatus     string `json:"old_status"`
	NewStatus     string `json:"new_status" binding:"required"`
	Message       string `json:"message"`
}
