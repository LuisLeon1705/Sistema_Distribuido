<?php

namespace Database\Seeders;

use App\Models\Category;
use Illuminate\Database\Seeder;
use Illuminate\Support\Str;

class CategorySeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        $categories = [
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ELEC',
                'nombre' => 'Electrónica',
                'descripcion' => 'Productos electrónicos y tecnología',
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ROPA',
                'nombre' => 'Ropa y Accesorios',
                'descripcion' => 'Prendas de vestir y accesorios de moda',
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'HOGAR',
                'nombre' => 'Hogar y Cocina',
                'descripcion' => 'Artículos para el hogar y cocina',
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'DEPORT',
                'nombre' => 'Deportes',
                'descripcion' => 'Artículos deportivos y fitness',
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'LIBROS',
                'nombre' => 'Libros y Medios',
                'descripcion' => 'Libros, música y películas',
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'JUGUET',
                'nombre' => 'Juguetes',
                'descripcion' => 'Juguetes y juegos para todas las edades',
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'AUTO',
                'nombre' => 'Automotriz',
                'descripcion' => 'Accesorios y repuestos para vehículos',
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'SALUD',
                'nombre' => 'Salud y Belleza',
                'descripcion' => 'Productos de cuidado personal y belleza',
                'created_at' => now(),
                'updated_at' => now(),
            ],
        ];

        foreach ($categories as $category) {
            Category::create($category);
        }

        $this->command->info('Categories seeded successfully!');
    }
}
