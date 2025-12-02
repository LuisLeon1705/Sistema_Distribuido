<?php

namespace App\Http\Controllers;

use App\Models\Category;
use Illuminate\Http\Request;

class CategoriasController extends Controller
{
    // -------------------------------- GET --------------------------------
    public function index()
    {
        return Category::all();
    }

    // -------------------------------- POST --------------------------------
    public function store(Request $request)
    {
        $request->validate([
            'nombre' => 'required|string|max:255',
            'descripcion' => 'nullable|string',
        ]);
        $prefijo = strtoupper(substr($request->nombre, 0, 3));
        if (strlen($prefijo) < 3) {
            $prefijo = str_pad($prefijo, 3, 'X');
        }
        while (Category::where('codigo', $prefijo)->exists()) {
            $ultimaLetra = substr($prefijo, -1);
            $parteInicial = substr($prefijo, 0, 2);
            if ($ultimaLetra === 'Z') {
                $ultimaLetra = 'A';
            } else {
                $ultimaLetra++;
            }
            $prefijo = $parteInicial . $ultimaLetra;
        }
        $data = $request->all();
        $data['codigo'] = $prefijo;
        $category = Category::create($data);
        return response()->json($category, 201);
    }

    // -------------------------------- GET por id --------------------------------
    public function show($id)
    {
        $category = Category::all()->find($id);
        if (!$category) {
            return response()->json(['mensaje' => 'Categoría no encontrado'], 404);
        }
        return response()->json($category, 200);
    }

    // -------------------------------- PUT --------------------------------
    public function update(Request $request, $id)
    {
        $category = Category::find($id);
        if (!$category) {
            return response()->json(['mensaje' => 'Categoría no encontrado'], 404);
        }
        $request->validate([
            'nombre' => 'string|max:255',
            'descripcion' => 'nullable|string',
        ]);
        $category->update($request->all());
        return response()->json([
            'mensaje' => 'Categoría actualizado',
            'Categoría' => $category
        ], 200);
    }

    // -------------------------------- DELETE --------------------------------
    public function destroy($id)
    {
        $category = Category::find($id);
        if (!$category) {
            return response()->json(['mensaje' => 'Categoría no encontrado'], 404);
        }
        $category->delete();
        return response()->json([
            'mensaje' => 'Categoría eliminada correctamente',
            "categoría" => $category
        ], 200);
    }
}
