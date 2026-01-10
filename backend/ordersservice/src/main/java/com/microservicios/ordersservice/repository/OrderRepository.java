package com.microservicios.ordersservice.repository;

import com.microservicios.ordersservice.model.Order;
import org.springframework.data.jpa.repository.JpaRepository;
import java.util.UUID;
import java.util.List;

public interface OrderRepository extends JpaRepository<Order, UUID> {
    List<Order> findByUserId(UUID userId);
}