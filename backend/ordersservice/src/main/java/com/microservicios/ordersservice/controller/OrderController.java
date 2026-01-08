package com.microservicios.ordersservice.controller;

import com.microservicios.ordersservice.model.Order;
import com.microservicios.ordersservice.repository.OrderRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;
import org.springframework.security.core.Authentication; 
import java.util.UUID;

import java.util.List;

@RestController
@RequestMapping("/api/orders") // Prefijo de la URL
public class OrderController {

    @Autowired
    private OrderRepository orderRepository;

    // GET /api/orders
    @GetMapping
    public List<Order> getAllOrders() {
        return orderRepository.findAll();
    }

    // POST /api/orders
    @PostMapping
    public Order createOrder(@RequestBody Order order, Authentication authentication) {
        
        UUID userId = (UUID) authentication.getPrincipal();
        order.setUserId(userId);
        order.setCreatedAt(java.time.LocalDateTime.now());
        order.setStatus("CREADO");
        if (order.getItems() != null) {
            order.addItems(order.getItems());
        }
        return orderRepository.save(order);
    }
}