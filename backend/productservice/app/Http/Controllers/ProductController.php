<?php

namespace App\Http\Controllers;

use App\Models\Product;
use Illuminate\Http\Request;

class ProductController extends Controller
{
    // -------------------------------- GET --------------------------------
    public function index()
    {
        // esto retorna los datos de la categoria tambien
        // return Product::with('categoria')->get();

        return Product::all();
    }

    // -------------------------------- POST --------------------------------
    public function store(Request $request)
    {
        $request->validate([
            'nombre' => 'required|string|max:255',
            'precio' => 'required|numeric',
            'id_categoria' => 'required|exists:categorias,id',
            'descripcion' => 'nullable|string',
            'imagen_url' => 'nullable|url'
        ]);
        $product = Product::create($request->all());
        return response()->json($product, 201);
    }

    /**
     * Display the specified resource.
     */
    public function show(string $id)
    {
        //
    }

    /**
     * Update the specified resource in storage.
     */
    public function update(Request $request, string $id)
    {
        //
    }

    /**
     * Remove the specified resource from storage.
     */
    public function destroy(string $id)
    {
        //
    }
}
