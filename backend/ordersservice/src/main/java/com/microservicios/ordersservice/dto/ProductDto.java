package com.microservicios.ordersservice.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.math.BigDecimal;

public class ProductDto {

    private String id;

    @JsonProperty("nombre")
    private String name;

    @JsonProperty("precio")
    private BigDecimal price;
    

    @JsonProperty("codigo")
    private String code;

    // --- Getters y Setters ---
    public String getId() { return id; }
    public void setId(String id) { this.id = id; }

    public String getName() { return name; }
    public void setName(String name) { this.name = name; }

    public BigDecimal getPrice() { return price; }
    public void setPrice(BigDecimal price) { this.price = price; }

    public String getCode() { return code; }
    public void setCode(String code) { this.code = code; }
}