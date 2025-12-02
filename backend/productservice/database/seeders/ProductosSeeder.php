<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use App\Models\Product;
use App\Models\Category;

class ProductosSeeder extends Seeder
{
    public function run(): void
    {
        $exampleProducts = [
            ['nombre' => 'Laptop Gamer', 'descripcion' => 'Laptop potente para gaming', 'precio' => 1299.99, 'categoria' => 'Electrónica', 'imagen_url' => 'https://via.placeholder.com/400x300?text=Laptop'],
            ['nombre' => 'Camiseta Negra', 'descripcion' => 'Camiseta de algodón', 'precio' => 19.95, 'categoria' => 'Ropa', 'imagen_url' => 'https://via.placeholder.com/400x300?text=Camiseta'],
            ['nombre' => 'Sofá 3 plazas', 'descripcion' => 'Sofá confortable para sala', 'precio' => 499.90, 'categoria' => 'Hogar', 'imagen_url' => 'https://via.placeholder.com/400x300?text=Sofa'],
            ['nombre' => 'Juego de Mesa', 'descripcion' => 'Diversión para toda la familia', 'precio' => 29.99, 'categoria' => 'Juguetes', 'imagen_url' => 'https://via.placeholder.com/400x300?text=Juego+de+Mesa'],
            ['nombre' => 'Novela: Las sombras', 'descripcion' => 'Una novela atrapante', 'precio' => 12.50, 'categoria' => 'Libros', 'imagen_url' => 'https://via.placeholder.com/400x300?text=Libro']
        ];

        foreach ($exampleProducts as $p) {
            $cat = Category::firstWhere('nombre', $p['categoria']);
            $catId = $cat ? $cat->id : null;
            if (!$catId) {
                $newCat = Category::create(['nombre' => $p['categoria']]);
                $catId = $newCat->id;
            }

            Product::updateOrCreate(
                ['nombre' => $p['nombre']],
                [
                    'id_categoria' => $catId,
                    'descripcion' => $p['descripcion'],
                    'precio' => $p['precio'],
                    'imagen_url' => $p['imagen_url'],
                    'estado' => true,
                ]
            );
        }
    }
}
