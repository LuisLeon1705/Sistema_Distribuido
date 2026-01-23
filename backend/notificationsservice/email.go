package main

import (
	"crypto/tls"
	"fmt"
	"log"
	"os"
	"strconv"

	"gopkg.in/gomail.v2"
)

// EmailConfig contiene la configuración SMTP
type EmailConfig struct {
	Host     string
	Port     int
	Username string
	Password string
	From     string
	UseTLS   bool
}

// GetEmailConfig obtiene la configuración de email desde las variables de entorno
func GetEmailConfig() *EmailConfig {
	port, _ := strconv.Atoi(os.Getenv("SMTP_PORT"))
	if port == 0 {
		port = 465
	}

	useTLS := os.Getenv("SMTP_USE_TLS") == "true"

	return &EmailConfig{
		Host:     os.Getenv("SMTP_HOST"),
		Port:     port,
		Username: os.Getenv("SMTP_USER"),
		Password: os.Getenv("SMTP_PASSWORD"),
		From:     os.Getenv("EMAIL_FROM"),
		UseTLS:   useTLS,
	}
}

// SendEmail envía un email usando la configuración SMTP
func SendEmail(to, subject, body string) error {
	config := GetEmailConfig()

	// Validar configuración
	if config.Host == "" || config.Username == "" {
		log.Println("⚠️  SMTP not configured, logging email instead")
		log.Printf("📧 Email Log:\n  To: %s\n  Subject: %s\n  Body: %s\n", to, subject, body)
		return nil
	}

	m := gomail.NewMessage()
	m.SetHeader("From", config.From)
	m.SetHeader("To", to)
	m.SetHeader("Subject", subject)
	m.SetBody("text/html", body)

	// Configurar dialer
	d := gomail.NewDialer(config.Host, config.Port, config.Username, config.Password)

	// Para SMTP con SSL/TLS directo (puerto 465)
	if config.Port == 465 {
		d.SSL = true
		d.TLSConfig = &tls.Config{
			InsecureSkipVerify: true,
			ServerName:         config.Host,
		}
	} else if config.UseTLS {
		// Para STARTTLS (puerto 587)
		d.TLSConfig = &tls.Config{
			InsecureSkipVerify: true,
			ServerName:         config.Host,
		}
	}

	// Enviar email
	if err := d.DialAndSend(m); err != nil {
		log.Printf("❌ Error sending email to %s: %v", to, err)
		return fmt.Errorf("failed to send email: %w", err)
	}

	log.Printf("✅ Email sent successfully to %s", to)
	return nil
}

// GenerateOrderCreatedEmail genera el HTML para el email de pedido creado
func GenerateOrderCreatedEmail(req OrderCreatedRequest) string {
	itemsHTML := ""
	for _, item := range req.Items {
		itemsHTML += fmt.Sprintf(`
			<tr>
				<td style="padding: 8px; border-bottom: 1px solid #ddd;">%s</td>
				<td style="padding: 8px; border-bottom: 1px solid #ddd; text-align: center;">%d</td>
				<td style="padding: 8px; border-bottom: 1px solid #ddd; text-align: right;">$%.2f</td>
			</tr>
		`, item.ProductName, item.Quantity, item.Price)
	}

	return fmt.Sprintf(`
		<!DOCTYPE html>
		<html>
		<head>
			<style>
				body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
				.container { max-width: 600px; margin: 0 auto; padding: 20px; }
				.header { background: #4CAF50; color: white; padding: 20px; text-align: center; border-radius: 5px 5px 0 0; }
				.content { background: #f9f9f9; padding: 20px; border-radius: 0 0 5px 5px; }
				.order-details { background: white; padding: 15px; margin: 20px 0; border-radius: 5px; }
				table { width: 100%%; border-collapse: collapse; margin-top: 10px; }
				th { background: #4CAF50; color: white; padding: 10px; text-align: left; }
				.total { font-size: 18px; font-weight: bold; color: #4CAF50; margin-top: 15px; text-align: right; }
			</style>
		</head>
		<body>
			<div class="container">
				<div class="header">
					<h1>🎉 ¡Pedido Confirmado!</h1>
				</div>
				<div class="content">
					<p>Hola <strong>%s</strong>,</p>
					<p>Tu pedido ha sido creado exitosamente. A continuación los detalles:</p>
					
					<div class="order-details">
						<p><strong>Número de Pedido:</strong> #%s</p>
						
						<table>
							<thead>
								<tr>
									<th>Producto</th>
									<th style="text-align: center;">Cantidad</th>
									<th style="text-align: right;">Precio</th>
								</tr>
							</thead>
							<tbody>
								%s
							</tbody>
						</table>
						
						<div class="total">
							Total: $%.2f
						</div>
					</div>
					
					<p>Recibirás actualizaciones sobre el estado de tu pedido.</p>
					<p>¡Gracias por tu compra!</p>
				</div>
			</div>
		</body>
		</html>
	`, req.CustomerName, req.OrderID, itemsHTML, req.TotalAmount)
}

// GenerateStatusChangeEmail genera el HTML para el email de cambio de estado
func GenerateStatusChangeEmail(req StatusChangeRequest) string {
	statusColor := "#4CAF50"
	statusEmoji := "📦"

	switch req.NewStatus {
	case "processing":
		statusColor = "#FF9800"
		statusEmoji = "⏳"
	case "shipped":
		statusColor = "#2196F3"
		statusEmoji = "🚚"
	case "delivered":
		statusColor = "#4CAF50"
		statusEmoji = "✅"
	case "cancelled":
		statusColor = "#F44336"
		statusEmoji = "❌"
	}

	message := req.Message
	if message == "" {
		message = fmt.Sprintf("El estado de tu pedido ha cambiado de <strong>%s</strong> a <strong>%s</strong>.", req.OldStatus, req.NewStatus)
	}

	return fmt.Sprintf(`
		<!DOCTYPE html>
		<html>
		<head>
			<style>
				body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
				.container { max-width: 600px; margin: 0 auto; padding: 20px; }
				.header { background: %s; color: white; padding: 20px; text-align: center; border-radius: 5px 5px 0 0; }
				.content { background: #f9f9f9; padding: 20px; border-radius: 0 0 5px 5px; }
				.status-box { background: white; padding: 20px; margin: 20px 0; border-radius: 5px; text-align: center; }
				.status-icon { font-size: 48px; margin-bottom: 10px; }
			</style>
		</head>
		<body>
			<div class="container">
				<div class="header">
					<h1>Actualización de Pedido</h1>
				</div>
				<div class="content">
					<div class="status-box">
						<div class="status-icon">%s</div>
						<h2>Pedido #%s</h2>
						<p>%s</p>
					</div>
					<p>Puedes consultar el estado de tu pedido en cualquier momento desde tu panel de usuario.</p>
					<p>¡Gracias por tu preferencia!</p>
				</div>
			</div>
		</body>
		</html>
	`, statusColor, statusEmoji, req.OrderID, message)
}

// GeneratePaymentRejectedEmail genera el HTML para el email de pago rechazado
func GeneratePaymentRejectedEmail(req PaymentRejectedRequest) string {
	reason := req.RejectionReason
	if reason == "" {
		reason = "El pago no pudo ser procesado por tu entidad bancaria."
	}

	transactionInfo := ""
	if req.TransactionID != "" {
		transactionInfo = fmt.Sprintf("<p><strong>ID de Transacción:</strong> %s</p>", req.TransactionID)
	}

	return fmt.Sprintf(`
		<!DOCTYPE html>
		<html>
		<head>
			<style>
				body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
				.container { max-width: 600px; margin: 0 auto; padding: 20px; }
				.header { background: #F44336; color: white; padding: 20px; text-align: center; border-radius: 5px 5px 0 0; }
				.content { background: #f9f9f9; padding: 20px; border-radius: 0 0 5px 5px; }
				.alert-box { background: #fff3cd; border: 1px solid #ffc107; padding: 20px; margin: 20px 0; border-radius: 5px; }
				.alert-icon { font-size: 48px; text-align: center; margin-bottom: 10px; }
				.amount { font-size: 24px; font-weight: bold; color: #F44336; text-align: center; margin: 15px 0; }
				.actions { background: white; padding: 20px; margin: 20px 0; border-radius: 5px; }
				.button { display: inline-block; background: #4CAF50; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; margin: 10px 5px; }
			</style>
		</head>
		<body>
			<div class="container">
				<div class="header">
					<h1>❌ Pago Rechazado</h1>
				</div>
				<div class="content">
					<p>Hola <strong>%s</strong>,</p>
					
					<div class="alert-box">
						<div class="alert-icon">⚠️</div>
						<p style="text-align: center;">Lamentamos informarte que tu pago no pudo ser procesado.</p>
						<div class="amount">Monto: $%.2f</div>
						<p style="text-align: center;"><strong>Pedido:</strong> #%s</p>
						%s
					</div>
					
					<p><strong>Motivo:</strong> %s</p>
					
					<div class="actions">
						<h3>¿Qué puedes hacer?</h3>
						<ul>
							<li>Verifica que tu tarjeta tenga fondos suficientes</li>
							<li>Asegúrate de que los datos de la tarjeta sean correctos</li>
							<li>Intenta con otro método de pago</li>
							<li>Contacta a tu banco si el problema persiste</li>
						</ul>
						<p style="text-align: center; margin-top: 20px;">
							<strong>Puedes intentar realizar el pago nuevamente desde tu panel de usuario.</strong>
						</p>
					</div>
					
					<p>Tu pedido ha sido marcado como <strong>FAILED</strong> y el inventario ha sido restaurado.</p>
					<p>Si necesitas ayuda, no dudes en contactarnos.</p>
				</div>
			</div>
		</body>
		</html>
	`, req.CustomerName, req.Amount, req.OrderID, transactionInfo, reason)
}
