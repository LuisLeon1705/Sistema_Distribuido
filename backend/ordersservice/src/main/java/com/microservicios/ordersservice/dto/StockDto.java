package com.microservicios.ordersservice.dto;

import com.fasterxml.jackson.annotation.JsonProperty;

public class StockDto {
    private Long id; // ID del registro de stock

    @JsonProperty("product_id")
    private String productId;

    @JsonProperty("quantity")
    private Integer quantity;

    @JsonProperty("warehouse_location")
    private String warehouseLocation;

    // Getters y Setters
    public Long getId() { return id; }
    public void setId(Long id) { this.id = id; }
    public String getProductId() { return productId; }
    public void setProductId(String productId) { this.productId = productId; }
    public Integer getQuantity() { return quantity; }
    public void setQuantity(Integer quantity) { this.quantity = quantity; }
    public String getWarehouseLocation() { return warehouseLocation; }
    public void setWarehouseLocation(String warehouseLocation) { this.warehouseLocation = warehouseLocation; }
}