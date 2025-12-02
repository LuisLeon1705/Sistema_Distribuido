<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use App\Models\Category;

class CategoriasSeeder extends Seeder
{
    public function run(): void
    {
        $categories = [
            ['nombre' => 'Electrónica', 'descripcion' => 'Dispositivos, gadgets y accesorios'],
            ['nombre' => 'Ropa', 'descripcion' => 'Moda y prendas de vestir'],
            ['nombre' => 'Hogar', 'descripcion' => 'Artículos para el hogar y decorativos'],
            ['nombre' => 'Libros', 'descripcion' => 'Libros, revistas y material de lectura'],
            ['nombre' => 'Juguetes', 'descripcion' => 'Juguetes y juegos para niños']
        ];

        foreach ($categories as $cat) {
            Category::updateOrCreate(['nombre' => $cat['nombre']], $cat);
        }
    }
}
