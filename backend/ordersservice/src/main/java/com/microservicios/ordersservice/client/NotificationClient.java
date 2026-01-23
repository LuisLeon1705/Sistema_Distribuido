package com.microservicios.ordersservice.client;

import com.microservicios.ordersservice.dto.notification.OrderCreatedNotificationRequest;
import com.microservicios.ordersservice.dto.notification.StatusChangeNotificationRequest;
import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;

@FeignClient(name = "notifications-service", url = "http://notificationsservice:8000")
public interface NotificationClient {

    @PostMapping("/notifications/order-created")
    void notifyOrderCreated(@RequestBody OrderCreatedNotificationRequest request);

    @PostMapping("/notifications/status-change")
    void notifyStatusChange(@RequestBody StatusChangeNotificationRequest request);
}
