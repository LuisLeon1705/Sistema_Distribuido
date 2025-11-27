<?php

namespace App\Http\Controllers;

use App\Models\Product;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Storage;

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
            'estado' => 'boolean',
            'imagen' => 'nullable|image|mimes:jpeg,png,jpg,webp|max:100000', // 100mb
        ]);
        $data = $request->except('imagen');
        if ($request->hasFile('imagen')) {
            $path = $request->file('imagen')->store('productos', 's3');
            $url = Storage::disk('s3')->url($path);
            $data['imagen_url'] = $url;
        }
        $product = Product::create($data);
        return response()->json([
            'mensaje' => 'Producto creado exitosamente',
            'producto' => $product
        ], 201);
    }

    // -------------------------------- GET por id  --------------------------------
    public function show($id)
    {
        $product = Product::all()->find($id);
        if (!$product) {
            return response()->json(['mensaje' => 'Producto no encontrado'], 404);
        }
        return response()->json($product, 200);
    }


    // -------------------------------- PUT --------------------------------
    public function update(Request $request, $id)
    {
        $product = Product::find($id);
        if (!$product) {
            return response()->json(['mensaje' => 'Producto no encontrado'], 404);
        }
        $request->validate([
            'nombre' => 'string|max:255',
            'precio' => 'numeric',
            'id_categoria' => 'exists:categorias,id',
            'descripcion' => 'nullable|string',
            'imagen_url' => 'nullable|url',
            'estado' => 'boolean'
        ]);
        $product->update($request->all());
        return response()->json([
            'mensaje' => 'Producto actualizado',
            'producto' => $product
        ], 200);
    }

    // -------------------------------- DELETE --------------------------------
    public function destroy($id)
    {
        $product = Product::find($id);
        if (!$product) {
            return response()->json(['mensaje' => 'Producto no encontrado'], 404);
        }
        $product->delete();
        return response()->json([
            'mensaje' => 'Producto eliminado correctamente',
            "producto" => $product
        ], 200);
    }

    // -------------------------------- GET por categoria --------------------------------
    public function getByCategory($id_categoria)
    {
        $productos = Product::where('id_categoria', $id_categoria)->get();
        return response()->json($productos, 200);
    }
    
    // -------------------------------- GET para activos (estado = true) --------------------------------
    public function getActive()
    {
        $productos = Product::where('estado', true)->get();
        return response()->json($productos, 200);
    }
}
