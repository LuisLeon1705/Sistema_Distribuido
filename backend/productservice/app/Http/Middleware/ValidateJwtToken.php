<?php

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Firebase\JWT\JWT;
use Firebase\JWT\Key;
use Symfony\Component\HttpFoundation\Response;
use Exception;

class ValidateJwtToken
{
    public function handle(Request $request, Closure $next): Response
    {
        $token = $request->bearerToken();

        if (!$token) {
            return response()->json(['message' => 'Token no proporcionado'], 401);
        }

        try {
            // Esto lanzará una excepción si el token es falso, expirado o la firma no coincide.
            $credentials = JWT::decode($token, new Key(env('JWT_SECRET'), env('JWT_ALGORITHM', 'HS256')));
            
            if (isset($credentials->role) && $credentials->role !== 'admin') {
                 // solo los admins pueden modificar producto
                return response()->json(['message' => 'Permisos insuficientes (Se requiere Admin)'], 403);
            }

            $request->merge([
                'user_id' => $credentials->sub,
                'user_role' => $credentials->role
            ]);

        } catch (Exception $e) {
            // Token expirado, firma inválida, etc.
            return response()->json(['message' => 'Token inválido o expirado: ' . $e->getMessage()], 401);
        }

        return $next($request);
    }
}