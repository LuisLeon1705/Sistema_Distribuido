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
                'nombre' => 'Bebidas', 
                'descripcion' => 'Bebidas para cada ocasión: jugos, refrescos y más.',
                'prefijo_codigo' => 'BEB',
                'created_at' => now(),
                'updated_at' => now()
            ],
            [
                'nombre' => 'Víveres', 
                'descripcion' => 'Productos básicos de tu despensa.',
                'prefijo_codigo' => 'VIB',
                'created_at' => now(),
                'updated_at' => now()
            ],
            [
                'nombre' => 'Dulces', 
                'descripcion' => 'Endulza tus momentos con caramelos, chocolates y golosinas irresistibles.',
                'prefijo_codigo' => 'DUL',
                'created_at' => now(),
                'updated_at' => now()
            ],
        ]);
    }
}