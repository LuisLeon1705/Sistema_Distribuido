package com.microservicios.ordersservice.client;

import com.microservicios.ordersservice.dto.UserDto;
import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestHeader;

import java.util.UUID;

@FeignClient(name = "auth-service", url = "http://authservice:8000")
public interface AuthClient {
    
    @GetMapping("/auth/me")
    UserDto getMe(@RequestHeader("Authorization") String token);

    @GetMapping("/users/{id}")
    UserDto getUserById(@RequestHeader("Authorization") String token, @PathVariable("id") UUID id);
}
