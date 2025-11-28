<?php

namespace Database\Seeders;

use App\Models\User;
use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use Illuminate\Database\Seeder;
use Database\Seeders\CategoriasSeeder;
use Database\Seeders\ProductosSeeder;

class DatabaseSeeder extends Seeder
{
    use WithoutModelEvents;

    /**
     * Seed the application's database.
     */
    public function run(): void
    {
        // You can populate users here as needed
        // User::factory(10)->create();

        // Seed categories and products
        $this->call([
            CategoriasSeeder::class,
            ProductosSeeder::class,
        ]);
    }
}
