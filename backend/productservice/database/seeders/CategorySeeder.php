<?php

namespace Database\Seeders;

use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use Illuminate\Database\Seeder;
use Illuminate\Support\Facades\DB;

class CategorySeeder extends Seeder
{
    public function run(): void
    {
        DB::table('categorias')->insert([
            [
                'id' => '11111111-1111-1111-1111-111111111111',
                'nombre' => 'Bebidas', 
                'descripcion' => 'Bebidas para cada ocasión: jugos, refrescos y más.',
                'codigo' => 'BEB',
                'created_at' => now(),
                'updated_at' => now()
            ],
            [
                'id' => '22222222-2222-2222-2222-222222222222',
                'nombre' => 'Víveres', 
                'descripcion' => 'Productos básicos de tu despensa.',
                'codigo' => 'VIB',
                'created_at' => now(),
                'updated_at' => now()
            ],
            [
                'id' => '33333333-3333-3333-3333-333333333333',
                'nombre' => 'Dulces', 
                'descripcion' => 'Endulza tus momentos con caramelos, chocolates y golosinas irresistibles.',
                'codigo' => 'DUL',
                'created_at' => now(),
                'updated_at' => now()
            ],
        ]);
    }
}