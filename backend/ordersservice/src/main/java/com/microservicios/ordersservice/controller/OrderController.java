package com.microservicios.ordersservice.controller;

import com.microservicios.ordersservice.client.InventoryClient;
import com.microservicios.ordersservice.client.ProductClient;
import com.microservicios.ordersservice.dto.ProductDto;
import com.microservicios.ordersservice.dto.OrderStatusDto;
import com.microservicios.ordersservice.dto.StockDto;
import com.microservicios.ordersservice.security.JwtUtil;
import com.microservicios.ordersservice.model.Order;
import com.microservicios.ordersservice.model.OrderItem;
import com.microservicios.ordersservice.repository.OrderRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.*;
import org.springframework.http.HttpStatus;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.http.ResponseEntity;

//import java.math.BigDecimal;
import java.util.List;
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

    @Autowired
    private InventoryClient inventoryClient;

    // ---------------------------------------------------------
    // 1. CREAR ORDEN (POST)
    // ---------------------------------------------------------
    @PostMapping
    @Transactional
    public Order createOrder(@RequestBody Order order, Authentication authentication) {
        UUID userId = (UUID) authentication.getPrincipal();
        order.setUserId(userId);

        Double totalCalculado = 0.0;

        if (order.getItems() != null) {
            for (OrderItem item : order.getItems()) {
                String productIdStr = item.getProductId().toString();
                ProductDto productReal = null;
                Double precioReal = 0.0;

                try {
                    // Validar producto y precio real (php)
                    productReal = productClient.getProductById(productIdStr);
                    
                    if (productReal == null) {
                        throw new RuntimeException("El producto " + item.getProductId() + " no existe.");
                    }
                    precioReal = productReal.getPrice().doubleValue();
                    item.setPrice(precioReal);
                    
                } catch (feign.FeignException.NotFound e) {
                    throw new RuntimeException("El producto con ID " + productIdStr + " no existe en el sistema.");
                }

                // validar stock en inventario (rust)
                List<StockDto> stocks = inventoryClient.getStockByProductId(productIdStr);
                if (stocks == null || stocks.isEmpty()) {
                    throw new RuntimeException("No hay registro de inventario para el producto " + productIdStr);
                }
                
                StockDto stockActual = stocks.get(0);
                
                if (stockActual.getQuantity() < item.getQuantity()) {
                    String nombreProd = (productReal != null) ? productReal.getName() : "Desconocido";
                    throw new RuntimeException("Stock insuficiente para el producto: " + nombreProd + 
                    ". Disponible: " + stockActual.getQuantity() + 
                    ", Solicitado: " + item.getQuantity());
                }

                Double subtotal = precioReal * item.getQuantity();
                totalCalculado += subtotal;
            }
        }

        order.setTotal(totalCalculado);
        order.setCreatedAt(java.time.LocalDateTime.now());
        order.setStatus("CREADO");

        if (order.getItems() != null) {
            order.addItems(order.getItems());
        }
        Order savedOrder = orderRepository.save(order);
        if (order.getItems() != null) {
            for (OrderItem item : order.getItems()) {
                String productIdStr = item.getProductId().toString();
                List<StockDto> stocks = inventoryClient.getStockByProductId(productIdStr);
                StockDto stockActual = stocks.get(0);
                int nuevaCantidad = stockActual.getQuantity() - item.getQuantity();
                StockDto stockUpdate = new StockDto();

                stockUpdate.setQuantity(nuevaCantidad);
                stockUpdate.setWarehouseLocation(stockActual.getWarehouseLocation());

                inventoryClient.updateStock(productIdStr, stockUpdate);
                System.out.println("----- Stock actualizado para " + productIdStr + ". Nuevo total: " + nuevaCantidad);
            }
        }
        return savedOrder;
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

    @PutMapping("/{id}/status")
    @Transactional
    public ResponseEntity<?> updateOrderStatus(
            @PathVariable UUID id,
            @RequestBody OrderStatusDto statusDto,
            @RequestHeader("Authorization") String tokenHeader) {
        
        String token = tokenHeader.replace("Bearer ", "");
        UUID userIdFromToken = jwtUtil.getUserIdFromToken(token);
        String role = jwtUtil.getRoleFromToken(token);
        String newStatus = statusDto.getStatus().toUpperCase();

        // buscar orden
        Order order = orderRepository.findById(id).orElse(null);

        if (order == null) {
            return ResponseEntity.status(404).body("Orden no encontrada.");
        }

        // validar permisos
        boolean isAdmin = "admin".equals(role);
        boolean isOwner = order.getUserId().equals(userIdFromToken);
        
        if (!isAdmin && isOwner) {
            return ResponseEntity.status(403).body("No tienes permiso para modificar esta orden.");
        }

        if (!isAdmin && isOwner) {
            if (!"CANCELLED".equals(newStatus)) {
                return ResponseEntity.status(403).body("Los usuarios solo pueden cancelar sus propias órdenes.");
            }
            if ("COMPLETED".equals(order.getStatus())) {
                return ResponseEntity.status(400).body("No se puede cancelar una orden ya completada.");
            }
        }

        // si se cancela para devolver el stock
        if ("CANCELLED".equals(newStatus) && !"CANCELLED".equals(order.getStatus())) {
            System.out.println(" ---- Orden Cancelada, devolviendo el stock");
            if (order.getItems() != null) {
                for (OrderItem item : order.getItems()) {
                    String productIdStr = item.getProductId().toString();
                    try {
                        List<StockDto> stocks = inventoryClient.getStockByProductId(productIdStr);
                        if (stocks != null && !stocks.isEmpty()) {
                            StockDto stockActual = stocks.get(0);
                            int nuevaCantidad = stockActual.getQuantity() + item.getQuantity();
                            StockDto stockUpdate = new StockDto();
                            stockUpdate.setQuantity(nuevaCantidad);
                            stockUpdate.setWarehouseLocation(stockActual.getWarehouseLocation());
                            inventoryClient.updateStock(productIdStr, stockUpdate);
                            System.out.println("----- Stock restaurado para " + productIdStr + ". Nuevo total: " + nuevaCantidad);
                        }
                    } catch (Exception e) {
                        System.out.println("Error al restaurar el stock para el producto " + productIdStr + ": " + e.getMessage());
                    }

                }
            }
        }
        order.setStatus(newStatus);
        orderRepository.save(order);
        return ResponseEntity.ok(order);
    }
}