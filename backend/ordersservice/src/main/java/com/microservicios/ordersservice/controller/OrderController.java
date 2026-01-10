package com.microservicios.ordersservice.controller;

import com.microservicios.ordersservice.client.ProductClient;
import com.microservicios.ordersservice.dto.ProductDto;
import com.microservicios.ordersservice.security.JwtUtil;
import com.microservicios.ordersservice.model.Order;
import com.microservicios.ordersservice.model.OrderItem;
import com.microservicios.ordersservice.repository.OrderRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.*;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import java.util.List;

//import java.math.BigDecimal;
import java.util.UUID;

@RestController
@RequestMapping("/api/orders")
public class OrderController {

    @Autowired
    private OrderRepository orderRepository;

    @Autowired
    private ProductClient productClient;

    @Autowired
    private JwtUtil jwtUtil;

    // ---------------------------------------------------------
    // 1. CREAR ORDEN (POST)
    // ---------------------------------------------------------
    @PostMapping
    public Order createOrder(@RequestBody Order order, Authentication authentication) {
        UUID userId = (UUID) authentication.getPrincipal();
        order.setUserId(userId);

        Double totalCalculado = 0.0;

        if (order.getItems() != null) {
            for (OrderItem item : order.getItems()) {
                String productIdStr = item.getProductId().toString();
                try {
                    ProductDto productReal = productClient.getProductById(productIdStr);
    
                    if (productReal == null) {
                        throw new RuntimeException("El producto " + item.getProductId() + " no existe.");
                    }
    
                    Double precioReal = productReal.getPrice().doubleValue();
                    item.setPrice(precioReal);
                    
                    Double subtotal = precioReal * item.getQuantity();
                    totalCalculado += subtotal;
                } catch (feign.FeignException.NotFound e) {
                    throw new RuntimeException("El producto con ID " + productIdStr + " no existe en el sistema.");
                }
            }
        }

        order.setTotal(totalCalculado);
        order.setCreatedAt(java.time.LocalDateTime.now());
        order.setStatus("CREADO");

        if (order.getItems() != null) {
            order.addItems(order.getItems());
        }

        return orderRepository.save(order);
    }

    // ---------------------------------------------------------
    // 2. OBTENER TODAS LAS ÓRDENES DEL USUARIO (GET)
    // ---------------------------------------------------------
    @GetMapping
    public ResponseEntity<List<Order>> getMyOrders(Authentication authentication) {
        UUID userId = (UUID) authentication.getPrincipal();
        return ResponseEntity.ok(orderRepository.findByUserId(userId));
    }

    // ---------------------------------------------------------
    // 3. OBTENER UNA ORDEN POR ID (GET /{id}) - CORREGIDO UUID
    // ---------------------------------------------------------
    @GetMapping("/{id}")
    public ResponseEntity<?> getOrderById(@PathVariable UUID id, Authentication authentication) { // <--- CAMBIO: UUID en vez de Long
        UUID userId = (UUID) authentication.getPrincipal();

        Order order = orderRepository.findById(id).orElse(null);

        if (order == null) {
            return ResponseEntity.status(404).body("Orden no encontrada");
        }

        if (!order.getUserId().equals(userId)) {
            return ResponseEntity.status(403).body("No tienes permiso para ver esta orden");
        }

        return ResponseEntity.ok(order);
    }

    // ---------------------------------------------------------
    // 4. ADMIN: VER TODAS LAS ÓRDENES DE TODOS (GET /all)
    // ---------------------------------------------------------
    @GetMapping("/all")
    public ResponseEntity<?> getAllOrdersAdmin(@RequestHeader("Authorization") String tokenHeader) {
        
        // 1. Limpiar el token
        String token = tokenHeader.replace("Bearer ", "");
        
        // 2. Obtener el rol
        String role = jwtUtil.getRoleFromToken(token);
        System.out.println("DEBUG: Intento de acceso admin. Rol detectado: " + role);

        // 3. Verificar si es admin
        if (role == null || !role.equals("admin")) {
            return ResponseEntity.status(HttpStatus.FORBIDDEN)
                    .body("Acceso denegado. Se requiere rol de administrador.");
        }

        // 4. Si es admin, devolver todo
        List<Order> allOrders = orderRepository.findAll();
        return ResponseEntity.ok(allOrders);
    }
}