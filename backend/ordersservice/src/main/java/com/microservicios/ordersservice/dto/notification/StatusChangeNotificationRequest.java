package com.microservicios.ordersservice.dto.notification;

import com.fasterxml.jackson.annotation.JsonProperty;

public class StatusChangeNotificationRequest {
    @JsonProperty("order_id")
    private String orderId;
    
    @JsonProperty("customer_email")
    private String customerEmail;
    
    @JsonProperty("old_status")
    private String oldStatus;
    
    @JsonProperty("new_status")
    private String newStatus;
    
    @JsonProperty("message")
    private String message;

    public StatusChangeNotificationRequest() {}

    public StatusChangeNotificationRequest(String orderId, String customerEmail, String oldStatus, String newStatus, String message) {
        this.orderId = orderId;
        this.customerEmail = customerEmail;
        this.oldStatus = oldStatus;
        this.newStatus = newStatus;
        this.message = message;
    }

    // Getters and Setters
    public String getOrderId() { return orderId; }
    public void setOrderId(String orderId) { this.orderId = orderId; }
    public String getCustomerEmail() { return customerEmail; }
    public void setCustomerEmail(String customerEmail) { this.customerEmail = customerEmail; }
    public String getOldStatus() { return oldStatus; }
    public void setOldStatus(String oldStatus) { this.oldStatus = oldStatus; }
    public String getNewStatus() { return newStatus; }
    public void setNewStatus(String newStatus) { this.newStatus = newStatus; }
    public String getMessage() { return message; }
    public void setMessage(String message) { this.message = message; }
}
