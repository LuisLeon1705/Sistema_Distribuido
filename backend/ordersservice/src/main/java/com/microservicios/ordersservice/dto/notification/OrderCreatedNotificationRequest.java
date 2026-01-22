package com.microservicios.ordersservice.dto.notification;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.math.BigDecimal;
import java.util.List;

public class OrderCreatedNotificationRequest {
    @JsonProperty("order_id")
    private String orderId;
    
    @JsonProperty("customer_email")
    private String customerEmail;
    
    @JsonProperty("customer_name")
    private String customerName;
    
    @JsonProperty("total_amount")
    private BigDecimal totalAmount;
    
    @JsonProperty("items")
    private List<NotificationItemDto> items;

    // Getters and Setters
    public String getOrderId() { return orderId; }
    public void setOrderId(String orderId) { this.orderId = orderId; }
    public String getCustomerEmail() { return customerEmail; }
    public void setCustomerEmail(String customerEmail) { this.customerEmail = customerEmail; }
    public String getCustomerName() { return customerName; }
    public void setCustomerName(String customerName) { this.customerName = customerName; }
    public BigDecimal getTotalAmount() { return totalAmount; }
    public void setTotalAmount(BigDecimal totalAmount) { this.totalAmount = totalAmount; }
    public List<NotificationItemDto> getItems() { return items; }
    public void setItems(List<NotificationItemDto> items) { this.items = items; }
}
