package main

import (
	"github.com/gin-gonic/gin"
)

// SetupRoutes configura todas las rutas de la aplicación
func SetupRoutes(router *gin.Engine) {
	// Grupo de rutas para notificaciones
	api := router.Group("/api")
	{
		// Rutas protegidas con JWT
		protected := api.Group("")
		protected.Use(JWTAuthMiddleware())
		{
			// POST /api/notifications - Crear notificación (requiere autenticación)
			protected.POST("/notifications", CreateNotification)

			// GET /api/notifications - Obtener todas las notificaciones (Admin/Staff)
			protected.GET("/notifications", RequireRole("admin", "staff"), GetNotifications)

			// GET /api/notifications/:id - Obtener notificación por ID
			protected.GET("/notifications/:id", GetNotificationByID)
		}
	}

	// Rutas públicas para ser llamadas desde otros microservicios
	// Estos endpoints deben ser protegidos a nivel de red (solo accesibles desde la red interna)
	router.POST("/notifications/order-created", NotifyOrderCreated)
	router.POST("/notifications/status-change", NotifyStatusChange)

	// Endpoint directo para crear notificaciones (sin autenticación, para microservicios)
	router.POST("/notifications", CreateNotification)

	// Endpoint para obtener todas las notificaciones (sin protección para facilitar testing)
	router.GET("/notifications", GetNotifications)
}
