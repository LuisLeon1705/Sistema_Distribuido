package com.microservicios.ordersservice;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.openfeign.EnableFeignClients;

@SpringBootApplication
@EnableFeignClients
public class OrdersserviceApplication {

	public static void main(String[] args) {
		SpringApplication.run(OrdersserviceApplication.class, args);
	}

}
