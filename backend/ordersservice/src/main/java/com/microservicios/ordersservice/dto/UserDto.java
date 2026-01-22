package com.microservicios.ordersservice.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.UUID;

public class UserDto {
    private UUID id;
    private String username;
    private String email;
    
    @JsonProperty("phone_number")
    private String phoneNumber;

    // Getters and Setters
    public UUID getId() { return id; }
    public void setId(UUID id) { this.id = id; }
    public String getUsername() { return username; }
    public void setUsername(String username) { this.username = username; }
    public String getEmail() { return email; }
    public void setEmail(String email) { this.email = email; }
    public String getPhoneNumber() { return phoneNumber; }
    public void setPhoneNumber(String phoneNumber) { this.phoneNumber = phoneNumber; }
}
