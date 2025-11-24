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


Route::get('/productos/categoria/{id}', [ProductController::class, 'getByCategory']);

Route::apiResource('/productos', ProductController::class);
Route::apiResource('/categorias', CategoriasController::class);