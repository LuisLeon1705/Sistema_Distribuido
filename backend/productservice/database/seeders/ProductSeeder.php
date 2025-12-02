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
            ['codigo' => 'BEB-000001', 'nombre' => 'Coca Cola Original', 'detalles' => '2 litros', 'precio' => 3.00, 'id_categoria' => '11111111-1111-1111-1111-111111111111', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/cocacola.jpg', 'descripcion' => 'Bebida gaseosa clásica'],
            ['codigo' => 'BEB-000002', 'nombre' => 'Jugo de Naranja', 'detalles' => '1 litro', 'precio' => 2.50, 'id_categoria' => '11111111-1111-1111-1111-111111111111', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/naranja.jpg', 'descripcion' => 'Saludable y natrual'],
            ['codigo' => 'BEB-000003', 'nombre' => 'Te de Manzana', 'detalles' => '750 ml', 'precio' => 5.00, 'id_categoria' => '11111111-1111-1111-1111-111111111111', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/manzana.jpg', 'descripcion' => 'Refrescante y delicioso'],
            ['codigo' => 'BEB-000004', 'nombre' => 'Arizona', 'detalles' => '350 ml', 'precio' => 1.25, 'id_categoria' => '11111111-1111-1111-1111-111111111111', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/arizona.png', 'descripcion' => 'Recarga energías con Arizona'],

            ['codigo' => 'VIB-000001', 'nombre' => 'Harina', 'detalles' => '1 kl', 'precio' => 1.10, 'id_categoria' => '22222222-2222-2222-2222-222222222222', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/harina.jpg', 'descripcion' => 'Ideal para repostería'],
            ['codigo' => 'VIB-000002', 'nombre' => 'Pasta', 'detalles' => '1 kl', 'precio' => 2.20, 'id_categoria' => '22222222-2222-2222-2222-222222222222', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/pasta.jpg', 'descripcion' => 'Fácil de cocinar'],
            ['codigo' => 'VIB-000003', 'nombre' => 'Aceite', 'detalles' => '750 ml', 'precio' => 4.35, 'id_categoria' => '22222222-2222-2222-2222-222222222222', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/aceite.jpg', 'descripcion' => 'Oliva extra virgen'],
            ['codigo' => 'VIB-000004', 'nombre' => 'Arroz', 'detalles' => '1 kl', 'precio' => 1.90, 'id_categoria' => '22222222-2222-2222-2222-222222222222', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/descarga.jpg', 'descripcion' => 'Grano largo premium'],

            ['codigo' => 'DUL-000001', 'nombre' => 'Galleta Oreo', 'detalles' => '150 mg', 'precio' => 2.50, 'id_categoria' => '33333333-3333-3333-3333-333333333333', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/oreo.jpg', 'descripcion' => 'Clásica y deliciosa'],
            ['codigo' => 'DUL-000002', 'nombre' => 'Chocolate Savoy', 'detalles' => '350 mg', 'precio' => 3.60, 'id_categoria' => '33333333-3333-3333-3333-333333333333', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/chocolate.jpg', 'descripcion' => 'Delicioso chocolate oscuro'],
            ['codigo' => 'DUL-000003', 'nombre' => 'Pringles', 'detalles' => '400 mg', 'precio' => 4.00, 'id_categoria' => '33333333-3333-3333-3333-333333333333', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/pringles.jpg', 'descripcion' => 'Papas fritas en tubo'],
            ['codigo' => 'DUL-000004', 'nombre' => 'Flips de Chocolate', 'detalles' => '150 mg', 'precio' => 2.30, 'id_categoria' => '33333333-3333-3333-3333-333333333333', 'imagen_url' => 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/flips.jpg', 'descripcion' => 'Crujientes y dulces'],
        ];

        foreach ($products as $product) {
            DB::table('productos')->insert([
                'id' => Str::uuid(),
                'codigo' => $product['codigo'], 
                'nombre' => $product['nombre'],
                'detalles' => $product['detalles'],
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