package com.microservicios.ordersservice.security;

import io.jsonwebtoken.Claims;
import io.jsonwebtoken.Jwts;
//import io.jsonwebtoken.security.SignatureException;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;
import javax.crypto.spec.SecretKeySpec;
import java.security.Key;
import java.nio.charset.StandardCharsets;
import java.util.UUID;

@Component
public class JwtUtil {

    @Value("${jwt.secret}")
    private String secret;

    private Key getSigningKey() {
        byte[] keyBytes = secret.getBytes(StandardCharsets.UTF_8);
        return new SecretKeySpec(keyBytes, "HmacSHA256");
    }

    public boolean validateToken(String token) {
        try {
            Jwts.parserBuilder()
                .setSigningKey(getSigningKey())
                .build()
                .parseClaimsJws(token);
            return true;
        } catch (Exception e) {
            return false;
        }
    }
    
    public UUID getUserIdFromToken(String token) {
        Claims claims = Jwts.parserBuilder()
                .setSigningKey(getSigningKey())
                .build()
                .parseClaimsJws(token)
                .getBody();
        return UUID.fromString(claims.getSubject());
    }

    public String getRoleFromToken(String token) {
        String[] parts = token.split("\\.");
        String unsignedToken = parts[0] + "." + parts[1] + ".";

        Claims claims = Jwts.parserBuilder()
                .build()
                .parseClaimsJwt(unsignedToken)
                .getBody();

        // Extraemos el campo "role" que puso Python
        return claims.get("role", String.class);
    }
}