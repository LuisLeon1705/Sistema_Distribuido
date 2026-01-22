package com.microservicios.ordersservice.dto.notification;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.math.BigDecimal;

public class NotificationItemDto {
    @JsonProperty("product_name")
    private String productName;
    
    @JsonProperty("quantity")
    private Integer quantity;
    
    @JsonProperty("price")
    private BigDecimal price;

    public NotificationItemDto() {}

    public NotificationItemDto(String productName, Integer quantity, BigDecimal price) {
        this.productName = productName;
        this.quantity = quantity;
        this.price = price;
    }

    public String getProductName() { return productName; }
    public void setProductName(String productName) { this.productName = productName; }
    public Integer getQuantity() { return quantity; }
    public void setQuantity(Integer quantity) { this.quantity = quantity; }
    public BigDecimal getPrice() { return price; }
    public void setPrice(BigDecimal price) { this.price = price; }
}
