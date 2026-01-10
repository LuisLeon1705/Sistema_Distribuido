package com.microservicios.ordersservice.client;

import com.microservicios.ordersservice.dto.StockDto;
import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.web.bind.annotation.*;

import java.util.List;

// URL interna de Docker: http://inventoryservice:3000
@FeignClient(name = "inventory-service", url = "http://inventoryservice:3000")
public interface InventoryClient {

    // Según tu documentación: GET /stock/{product_id} devuelve un Array []
    @GetMapping("/stock/{productId}")
    List<StockDto> getStockByProductId(@PathVariable("productId") String productId);

    // Según tu documentación: PUT /stock/{product_id} actualiza la cantidad
    @PutMapping("/stock/{productId}")
    StockDto updateStock(@PathVariable("productId") String productId, @RequestBody StockDto stockDto);
}