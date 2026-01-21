package com.microservicios.ordersservice.exception;

import feign.FeignException;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

import java.util.HashMap;
import java.util.Map;

@RestControllerAdvice
public class GlobalExceptionHandler {

    // CASO 1: El microservicio de Productos (PHP) responde con 404
    @ExceptionHandler(FeignException.NotFound.class)
    public ResponseEntity<Map<String, String>> handleFeignNotFound(FeignException e) {
        Map<String, String> response = new HashMap<>();
        response.put("error", "Producto no encontrado");
        response.put("mensaje", "El ID del producto solicitado no existe en el catálogo externo.");
        // Opcional: extrae detalles técnicos si quieres
        // response.put("detalle", e.getMessage()); 
        
        return new ResponseEntity<>(response, HttpStatus.NOT_FOUND);
    }

    // CASO 2: Errores genéricos o validaciones manuales (RuntimeException)
    @ExceptionHandler(RuntimeException.class)
    public ResponseEntity<Map<String, String>> handleRuntimeException(RuntimeException e) {
        Map<String, String> response = new HashMap<>();
        response.put("error", "Error en la solicitud");
        response.put("mensaje", e.getMessage());
        
        return new ResponseEntity<>(response, HttpStatus.BAD_REQUEST);
    }
    
    // CASO 3: Error de conexión (PHP está apagado)
    @ExceptionHandler(FeignException.ServiceUnavailable.class)
    public ResponseEntity<Map<String, String>> handleServiceUnavailable(FeignException e) {
        Map<String, String> response = new HashMap<>();
        response.put("error", "Servicio no disponible");
        response.put("mensaje", "No se pudo conectar con el catálogo de productos. Intente más tarde.");
        
        return new ResponseEntity<>(response, HttpStatus.SERVICE_UNAVAILABLE);
    }
}