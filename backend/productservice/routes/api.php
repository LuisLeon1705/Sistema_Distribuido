<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\ProductController;
use App\Http\Controllers\CategoriasController;

Route::get('/user', function (Request $request) {
    return $request->user();
})->middleware('auth:sanctum');

Route::get('/prueba', function () {
    return response()->json([
        'Mensajes' => 'Hola mundo'
    ]);
});

Route::get('/productos', [ProductController::class, 'index']);
Route::get('/productos/activos', [ProductController::class, 'getActive']);
Route::get('/productos/{id}', [ProductController::class, 'show']);
Route::get('/productos/codigo/{codigo}', [ProductController::class, 'getByCode']);
Route::get('/productos/categoria/{id}', [ProductController::class, 'getByCategory']);
Route::get('/categorias', [CategoriasController::class, 'index']);
Route::get('/categorias/{id}', [CategoriasController::class, 'show']);

// Productos
Route::post('/productos', [ProductController::class, 'store']);
Route::put('/productos/{id}', [ProductController::class, 'update']);
Route::delete('/productos/{id}', [ProductController::class, 'destroy']);

// Categorías
Route::post('/categorias', [CategoriasController::class, 'store']);
Route::put('/categorias/{id}', [CategoriasController::class, 'update']);
Route::delete('/categorias/{id}', [CategoriasController::class, 'destroy']);
Route::middleware(['jwt.verify'])->group(function () {
});