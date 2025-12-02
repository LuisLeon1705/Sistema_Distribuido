<?php

namespace Database\Seeders;

use Illuminate\Database\Seeder;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

class ProductSeeder extends Seeder
{
    public function run(): void
    {
        $products = [
            ['codigo' => 'BEB-001', 'nombre' => 'Coca Cola 2 litros', 'precio' => 3.00, 'id_categoria' => 1, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/cocacola.jpg', 'descripcion' => 'Bebida gaseosa clásica'],
            ['codigo' => 'BEB-002', 'nombre' => 'Jugo de Naranja', 'precio' => 2.50, 'id_categoria' => 1, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/naranja.jpg', 'descripcion' => 'Saludable y natrual'],
            ['codigo' => 'BEB-003', 'nombre' => 'Te de Manzana', 'precio' => 5.00, 'id_categoria' => 1, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/manzana.jpg', 'descripcion' => 'Refrescante y delicioso'],
            ['codigo' => 'BEB-004', 'nombre' => 'Arizona 350ml', 'precio' => 1.25, 'id_categoria' => 1, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/arizona.png', 'descripcion' => 'Recarga energías con Arizona'],

            ['codigo' => 'VIB-001', 'nombre' => 'Harina 1kl', 'precio' => 1.10, 'id_categoria' => 2, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/harina.jpg', 'descripcion' => 'Ideal para repostería'],
            ['codigo' => 'VIB-002', 'nombre' => 'Pasta 1kl', 'precio' => 2.20, 'id_categoria' => 2, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/pasta.jpg', 'descripcion' => 'Fácil de cocinar'],
            ['codigo' => 'VIB-003', 'nombre' => 'Aceite 750ml', 'precio' => 4.35, 'id_categoria' => 2, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/aceite.jpg', 'descripcion' => 'Oliva extra virgen'],
            ['codigo' => 'VIB-004', 'nombre' => 'Arroz 1kl', 'precio' => 1.90, 'id_categoria' => 2, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/descarga.jpg', 'descripcion' => 'Grano largo premium'],

            ['codigo' => 'DUL-001', 'nombre' => 'Galleta Oreo', 'precio' => 2.50, 'id_categoria' => 3, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/oreo.jpg', 'descripcion' => 'Clásica y deliciosa'],
            ['codigo' => 'DUL-002', 'nombre' => 'Chocolate Savoy', 'precio' => 3.60, 'id_categoria' => 3, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/chocolate.jpg', 'descripcion' => 'Delicioso chocolate oscuro'],
            ['codigo' => 'DUL-003', 'nombre' => 'Pringles', 'precio' => 4.00, 'id_categoria' => 3, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/pringles.jpg', 'descripcion' => 'Papas fritas en tubo'],
            ['codigo' => 'DUL-004', 'nombre' => 'Flips de Chocolate', 'precio' => 2.30, 'id_categoria' => 3, 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/flips.jpg', 'descripcion' => 'Crujientes y dulces'],
        ];

        foreach ($products as $product) {
            DB::table('productos')->insert([
                'id' => Str::uuid(),
                'codigo' => $product['codigo'], 
                'nombre' => $product['nombre'],
                'precio' => $product['precio'],
                'id_categoria' => $product['id_categoria'],
                'descripcion' => $product['descripcion'],
                'imagen_url' => $product['imagen_url'],
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ]);
        }
    }
}