<?php

namespace App\Http\Controllers;

use App\Models\Product;
use App\Models\Category;
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
                'detalles' => 'nullable|string|max:255',
                'precio' => 'required|numeric',
                // Accept either `id_categoria` or `categoria_id` to be compatible with frontend
                'id_categoria' => ['sometimes', \Illuminate\Validation\Rule::exists(Category::class, 'id')],
                'categoria_id' => ['sometimes', \Illuminate\Validation\Rule::exists(Category::class, 'id')],
                'descripcion' => 'nullable|string',
                // Accept image file OR url
                'imagen' => 'nullable|image|mimes:jpeg,png,jpg,webp|max:100000', // 100mb
                'imagen_url' => 'nullable|url',
                // Accept estado string or boolean; we'll normalize it below
                'estado' => 'nullable',
            ]);
            $data = $request->except('imagen');
            if ($request->hasFile('imagen')) {
                $path = $request->file('imagen')->store('productos', 's3');
                if (!$path) {
                    // Si entra aquí, es que la conexión con AWS falló
                    return response()->json([
                        'error' => 'No se pudo subir la imagen a S3.',
                        'debug_hint' => 'Revisa tus credenciales en .env o el nombre del bucket.'
                    ], 500);
                }
                $url = Storage::disk('s3')->url($path);
                $data['imagen_url'] = $url;
            }
            // If frontend sent a plain url for the image under 'imagen_url' or 'imagen', support it
            if ($request->filled('imagen_url') && !isset($data['imagen_url'])) {
                $data['imagen_url'] = $request->input('imagen_url');
            }
            if ($request->filled('imagen') && !isset($data['imagen_url']) && ! $request->hasFile('imagen')) {
                // If frontend sent 'imagen' as a url string, map it
                $data['imagen_url'] = $request->input('imagen');
            }

            // Support both naming conventions for category id
            if ($request->filled('categoria_id') && !isset($data['id_categoria'])) {
                $data['id_categoria'] = $request->input('categoria_id');
            }
            if (!isset($data['id_categoria'])) {
                return response()->json(['error' => 'La categoría es obligatoria para generar el código.'], 422);
            }
            $categoria = Category::find($data['id_categoria']);
            if (!$categoria) {
                return response()->json(['error' => 'Categoría no encontrada.'], 404);
            }
            $prefijo = $categoria && $categoria->codigo ? strtoupper($categoria->codigo) : 'DEF';
            do {
                $numeroAleatorio = mt_rand(0, 999999);
                $numeroString = str_pad($numeroAleatorio, 6, '0', STR_PAD_LEFT);
                $codigoGenerado = $prefijo . '-' . $numeroString;
            } while (Product::where('codigo', $codigoGenerado)->exists());
            $data['codigo'] = $codigoGenerado;
            // Normalize estado: accept 'activo'/'inactivo', booleans or strings
            if (isset($data['estado'])) {
                $estadoValue = $data['estado'];
                if (is_string($estadoValue)) {
                    $data['estado'] = strtolower($estadoValue) === 'activo' || strtolower($estadoValue) === 'true';
                } else {
                    $data['estado'] = (bool)$estadoValue;
                }
            } else {
                $data['estado'] = true;
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
        $product = Product::find($id);
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
            'detalles' => 'nullable|string|max:255',
            'precio' => 'numeric',
            'id_categoria' => 'sometimes|exists:categorias,id',
            'categoria_id' => 'sometimes|exists:categorias,id',
            'descripcion' => 'nullable|string',
            'imagen' => 'nullable|image|mimes:jpeg,png,jpg,webp|max:100000',
            'imagen_url' => 'nullable|url',
            'estado' => 'nullable'
        ]);
        $data = $request->except('imagen');
        // If new file provided, upload it
        if ($request->hasFile('imagen')) {
            $path = $request->file('imagen')->store('productos', 's3');
            $url = Storage::disk('s3')->url($path);
            $data['imagen_url'] = $url;
        }
        // Support plain image URLs in 'imagen_url' or 'imagen' string
        if ($request->filled('imagen_url') && !isset($data['imagen_url'])) {
            $data['imagen_url'] = $request->input('imagen_url');
        }
        if ($request->filled('imagen') && !isset($data['imagen_url']) && ! $request->hasFile('imagen')) {
            $data['imagen_url'] = $request->input('imagen');
        }
        // Support both naming conventions for category id
        if ($request->filled('categoria_id')) {
            $data['id_categoria'] = $request->input('categoria_id');
        }
        if ($request->filled('id_categoria') || $request->filled('categoria_id')) {
            $categoria = Category::find($data['id_categoria']);
            if (!$categoria) {
                return response()->json(['error' => 'Categoría no encontrada.'], 404);
            }
            $prefijo = $categoria && $categoria->codigo ? strtoupper($categoria->codigo) : 'DEF';
            do {
                $numeroAleatorio = mt_rand(0, 999999);
                $numeroString = str_pad($numeroAleatorio, 6, '0', STR_PAD_LEFT);
                $codigoGenerado = $prefijo . '-' . $numeroString;
            } while (Product::where('codigo', $codigoGenerado)->exists());
            $data['codigo'] = $codigoGenerado;
        }

        // Normalize estado
        if (isset($data['estado'])) {
            $estadoValue = $data['estado'];
            if (is_string($estadoValue)) {
                $data['estado'] = strtolower($estadoValue) === 'activo' || strtolower($estadoValue) === 'true';
            } else {
                $data['estado'] = (bool)$estadoValue;
            }
        }
        $product->update($data);
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
    
    
    // -------------------------------- GET por Codigo --------------------------------
    public function getByCode($codigo)
    {
        $productos = Product::where('codigo', $codigo)->get();
        return response()->json($productos, 200);
    }
    
    // -------------------------------- GET para activos (estado = true) --------------------------------
    public function getActive()
    {
        $productos = Product::where('estado', true)->get();
        return response()->json($productos, 200);
    }
}
