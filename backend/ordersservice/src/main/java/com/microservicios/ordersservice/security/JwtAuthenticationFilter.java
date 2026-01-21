package com.microservicios.ordersservice.security;

import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;

import java.io.IOException;
import java.util.ArrayList;
import java.util.UUID;

@Component
public class JwtAuthenticationFilter extends OncePerRequestFilter {

    @Autowired
    private JwtUtil jwtUtil;

    @Override
    protected void doFilterInternal(HttpServletRequest request, 
                                    HttpServletResponse response, 
                                    FilterChain filterChain)
            throws ServletException, IOException {

        // --- LOGS DE DEBUGGING (INICIO) ---
        System.out.println("\n=== NUEVA PETICIÓN ENTRANTE ===");
        System.out.println("URL: " + request.getRequestURI());
        System.out.println("Método: " + request.getMethod());
        
        String authHeader = request.getHeader("Authorization");
        System.out.println("Header Authorization recibido: " + authHeader);

        if (authHeader != null && authHeader.startsWith("Bearer ")) {
            String token = authHeader.substring(7); // Quitar "Bearer "
            
            // Imprimir un pedacito del token para verificar que no esté vacío
            String tokenPreview = (token.length() > 10) ? token.substring(0, 10) + "..." : token;
            System.out.println("Token extraído: " + tokenPreview);

            try {
                // Validar Token
                if (jwtUtil.validateToken(token)) {
                    System.out.println("✅ Token Válido. Extrayendo usuario...");
                    
                    UUID userId = jwtUtil.getUserIdFromToken(token);
                    System.out.println("Usuario ID: " + userId);

                    // Autenticar en Spring Security
                    UsernamePasswordAuthenticationToken authToken = 
                            new UsernamePasswordAuthenticationToken(userId, null, new ArrayList<>());
                    
                    SecurityContextHolder.getContext().setAuthentication(authToken);
                    System.out.println("✅ Usuario autenticado en el contexto de seguridad.");
                } else {
                    System.out.println("❌ Token Inválido según JwtUtil.");
                }
            } catch (Exception e) {
                System.out.println("❌ Error validando token: " + e.getMessage());
                e.printStackTrace();
            }
        } else {
            System.out.println("⚠️ No hay header Authorization o no empieza con 'Bearer '");
        }

        System.out.println("=== CONTINUANDO FILTRO ===\n");
        // --- LOGS DE DEBUGGING (FIN) ---

        // Continuar con la cadena de filtros (ir al Controller)
        filterChain.doFilter(request, response);
    }
}