package com.microservicios.ordersservice.client;

import com.microservicios.ordersservice.dto.ProductDto;
import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;

@FeignClient(name = "product-service", url = "http://productservice")
public interface ProductClient {

    @GetMapping("/api/productos/{id}")
    ProductDto getProductById(@PathVariable("id") String id);
}