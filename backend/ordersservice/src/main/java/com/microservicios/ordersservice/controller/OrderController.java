package com.microservicios.ordersservice.controller;

import com.microservicios.ordersservice.client.InventoryClient;
import com.microservicios.ordersservice.client.ProductClient;
import com.microservicios.ordersservice.client.AuthClient;
import com.microservicios.ordersservice.client.NotificationClient;
import com.microservicios.ordersservice.dto.ProductDto;
import com.microservicios.ordersservice.dto.OrderStatusDto;
import com.microservicios.ordersservice.dto.StockDto;
import com.microservicios.ordersservice.dto.UserDto;
import com.microservicios.ordersservice.dto.notification.OrderCreatedNotificationRequest;
import com.microservicios.ordersservice.dto.notification.StatusChangeNotificationRequest;
import com.microservicios.ordersservice.dto.notification.NotificationItemDto;
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

import java.math.BigDecimal;
import java.util.List;
import java.util.UUID;
import java.util.ArrayList;

@RestController
@RequestMapping({"/api/orders", "/api/orders/"})
public class OrderController {

    @Autowired
    private OrderRepository orderRepository;

    @Autowired
    private ProductClient productClient;

    @Autowired
    private JwtUtil jwtUtil;

    @Autowired
    private InventoryClient inventoryClient;

    @Autowired
    private AuthClient authClient;

    @Autowired
    private NotificationClient notificationClient;

    // ---------------------------------------------------------
    // 1. CREAR ORDEN (POST)
    // ---------------------------------------------------------
    @PostMapping
    @Transactional
    public Order createOrder(@RequestBody Order order, Authentication authentication, @RequestHeader("Authorization") String token) {
        System.out.println(">>> Entrando a createOrder");
        UUID userId = (UUID) authentication.getPrincipal();
        order.setUserId(userId);

        Double totalCalculado = 0.0;
        List<NotificationItemDto> notificationItems = new ArrayList<>();

        if (order.getItems() != null) {
            for (OrderItem item : order.getItems()) {
                String productIdStr = item.getProductId().toString();
                System.out.println(">>> Verificando producto: " + productIdStr);
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
                System.out.println(">>> Verificando stock para: " + productIdStr);
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

                // Add to notification items
                notificationItems.add(new NotificationItemDto(
                    productReal.getName(),
                    item.getQuantity(),
                    BigDecimal.valueOf(precioReal)
                ));
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

        // --- NOTIFICACIÓN ---
        try {
            System.out.println(">>> Enviando notificación...");
            UserDto user = authClient.getMe(token);
            OrderCreatedNotificationRequest notification = new OrderCreatedNotificationRequest();
            notification.setOrderId(savedOrder.getId().toString());
            notification.setCustomerEmail(user.getEmail());
            notification.setCustomerName(user.getUsername());
            notification.setTotalAmount(BigDecimal.valueOf(totalCalculado));
            notification.setItems(notificationItems);

            notificationClient.notifyOrderCreated(notification);
            System.out.println("✅ Notificación de orden creada enviada para " + user.getEmail());
        } catch (Exception e) {
            System.err.println("❌ Error enviando notificación de orden creada: " + e.getMessage());
            // No lanzamos excepción para no revertir la orden
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
        
        if (!isAdmin && !isOwner) {
            return ResponseEntity.status(403).body("No tienes permiso para modificar esta orden.");
        }

        // Evitar cambios en órdenes canceladas
        if ("CANCELADO".equals(order.getStatus())) {
            return ResponseEntity.status(400).body("No se puede cambiar el status de una orden ya cancelada.");
        }

        // Evitar cambios en órdenes canceladas
        if ("ENVIADO".equals(order.getStatus())) {
            return ResponseEntity.status(400).body("No se puede cambiar el status de una orden ya enviada.");
        }

        // si se cancela o falla el pago, devolver el stock
        if (("CANCELADO".equals(newStatus) || "FAILED".equals(newStatus)) && 
            !"CANCELADO".equals(order.getStatus()) && 
            !"FAILED".equals(order.getStatus())) {
            
            System.out.println(" ---- Orden Cancelada/Fallida, devolviendo el stock");
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
        String oldStatus = order.getStatus();
        order.setStatus(newStatus);
        orderRepository.save(order);

        // --- NOTIFICACIÓN ---
        try {
            // Obtener email del destinatario (dueño de la orden)
            String recipientEmail = null;
            
            if (isAdmin) {
                // Si es admin, obtenemos los datos del usuario dueño de la orden por su ID
                UserDto user = authClient.getUserById(token, order.getUserId());
                recipientEmail = user.getEmail();
            } else {
                // Si es el usuario dueño, usamos getMe
                UserDto user = authClient.getMe(token);
                recipientEmail = user.getEmail();
            }

            if (recipientEmail != null) {
                StatusChangeNotificationRequest notification = new StatusChangeNotificationRequest(
                    order.getId().toString(),
                    recipientEmail,
                    oldStatus,
                    newStatus,
                    "El estado de tu orden ha cambiado a " + newStatus
                );
                notificationClient.notifyStatusChange(notification);
                System.out.println("✅ Notificación de cambio de estado enviada a " + recipientEmail);
            }
        } catch (Exception e) {
            System.err.println("❌ Error enviando notificación de cambio de estado: " + e.getMessage());
        }

        return ResponseEntity.ok(order);
    }
}