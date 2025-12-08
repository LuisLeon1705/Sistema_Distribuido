<?php

namespace Database\Seeders;

use App\Models\Product;
use App\Models\Category;
use Illuminate\Database\Seeder;
use Illuminate\Support\Str;

class ProductSeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        // Obtener las categorías por código
        $electronica = Category::where('codigo', 'ELEC')->first();
        $ropa = Category::where('codigo', 'ROPA')->first();
        $hogar = Category::where('codigo', 'HOGAR')->first();
        $deportes = Category::where('codigo', 'DEPORT')->first();
        $libros = Category::where('codigo', 'LIBROS')->first();
        $juguetes = Category::where('codigo', 'JUGUET')->first();
        $auto = Category::where('codigo', 'AUTO')->first();
        $salud = Category::where('codigo', 'SALUD')->first();

        $products = [
            // Electrónica
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ELEC001',
                'nombre' => 'Laptop HP 15.6"',
                'detalles' => 'Intel Core i5, 8GB RAM, 256GB SSD',
                'descripcion' => 'Laptop HP de 15.6 pulgadas con procesador Intel Core i5, 8GB de RAM y 256GB de almacenamiento SSD. Perfecta para trabajo y estudios.',
                'precio' => 599.99,
                'id_categoria' => $electronica->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1496181133206-80ce9b88a853?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ELEC002',
                'nombre' => 'Mouse Inalámbrico Logitech',
                'detalles' => 'Conexión USB, batería de larga duración',
                'descripcion' => 'Mouse inalámbrico ergonómico con sensor de precisión y batería que dura hasta 18 meses.',
                'precio' => 24.99,
                'id_categoria' => $electronica->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1527864550417-7fd91fc51a46?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ELEC003',
                'nombre' => 'Auriculares Bluetooth Sony',
                'detalles' => 'Cancelación de ruido, 30h de batería',
                'descripcion' => 'Auriculares inalámbricos con cancelación activa de ruido y hasta 30 horas de reproducción continua.',
                'precio' => 149.99,
                'id_categoria' => $electronica->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ELEC004',
                'nombre' => 'Teclado Mecánico RGB',
                'detalles' => 'Switches azules, retroiluminación RGB',
                'descripcion' => 'Teclado mecánico para gaming con switches azules, retroiluminación RGB personalizable y reposamanos.',
                'precio' => 89.99,
                'id_categoria' => $electronica->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1587829741301-dc798b83add3?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],

            // Ropa y Accesorios
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ROPA001',
                'nombre' => 'Camiseta Básica de Algodón',
                'detalles' => '100% algodón, colores variados',
                'descripcion' => 'Camiseta clásica de algodón 100%, disponible en varios colores. Perfecta para el uso diario.',
                'precio' => 15.99,
                'id_categoria' => $ropa->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1521572163474-6864f9cf17ab?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ROPA002',
                'nombre' => 'Jeans Slim Fit',
                'detalles' => 'Denim elástico, corte moderno',
                'descripcion' => 'Jeans de corte slim fit con tela de denim elástico para mayor comodidad y estilo.',
                'precio' => 49.99,
                'id_categoria' => $ropa->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1542272604-787c3835535d?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'ROPA003',
                'nombre' => 'Zapatillas Deportivas',
                'detalles' => 'Suela de goma, diseño transpirable',
                'descripcion' => 'Zapatillas deportivas con diseño moderno, suela de goma antideslizante y material transpirable.',
                'precio' => 69.99,
                'id_categoria' => $ropa->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1460353581641-37baddab0fa2?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],

            // Hogar y Cocina
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'HOGAR001',
                'nombre' => 'Cafetera Automática',
                'detalles' => 'Capacidad 1.8L, función timer',
                'descripcion' => 'Cafetera programable con capacidad de 1.8 litros, función timer y plato térmico para mantener el café caliente.',
                'precio' => 79.99,
                'id_categoria' => $hogar->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1517668808822-9ebb02f2a0e6?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'HOGAR002',
                'nombre' => 'Juego de Sartenes Antiadherentes',
                'detalles' => 'Set de 3 piezas, libre de PFOA',
                'descripcion' => 'Set de 3 sartenes antiadherentes de diferentes tamaños, libres de PFOA y aptas para lavavajillas.',
                'precio' => 44.99,
                'id_categoria' => $hogar->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1556909114-f6e7ad7d3136?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'HOGAR003',
                'nombre' => 'Aspiradora Robot',
                'detalles' => 'Navegación inteligente, WiFi',
                'descripcion' => 'Aspiradora robot con navegación inteligente, control por WiFi y programación mediante app móvil.',
                'precio' => 299.99,
                'id_categoria' => $hogar->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1558317374-067fb5f30001?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],

            // Deportes
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'DEPORT001',
                'nombre' => 'Pesas Ajustables 20kg',
                'detalles' => 'Par de mancuernas, peso ajustable',
                'descripcion' => 'Set de pesas ajustables de hasta 20kg cada una, perfectas para entrenamiento en casa.',
                'precio' => 89.99,
                'id_categoria' => $deportes->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1517836357463-d25dfeac3438?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'DEPORT002',
                'nombre' => 'Esterilla de Yoga',
                'detalles' => 'Antideslizante, grosor 6mm',
                'descripcion' => 'Esterilla de yoga antideslizante de 6mm de grosor, incluye correa de transporte.',
                'precio' => 29.99,
                'id_categoria' => $deportes->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1601925260368-ae2f83cf8b7f?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'DEPORT003',
                'nombre' => 'Bicicleta de Montaña',
                'detalles' => '21 velocidades, suspensión delantera',
                'descripcion' => 'Bicicleta de montaña con 21 velocidades, suspensión delantera y cuadro de aluminio resistente.',
                'precio' => 399.99,
                'id_categoria' => $deportes->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1511994298241-608e28f14fde?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],

            // Libros y Medios
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'LIBROS001',
                'nombre' => 'El Principito - Antoine de Saint-Exupéry',
                'detalles' => 'Tapa dura, edición ilustrada',
                'descripcion' => 'Edición especial de El Principito con ilustraciones a color y tapa dura.',
                'precio' => 19.99,
                'id_categoria' => $libros->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1512820790803-83ca734da794?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'LIBROS002',
                'nombre' => 'E-Reader Kindle',
                'detalles' => 'Pantalla 6", luz integrada',
                'descripcion' => 'Lector electrónico con pantalla de 6 pulgadas, luz integrada ajustable y batería de larga duración.',
                'precio' => 129.99,
                'id_categoria' => $libros->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1592496001020-d31bd830651f?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],

            // Juguetes
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'JUGUET001',
                'nombre' => 'Set de Bloques de Construcción',
                'detalles' => '500 piezas, compatible con LEGO',
                'descripcion' => 'Set de 500 bloques de construcción en colores variados, compatible con otras marcas populares.',
                'precio' => 34.99,
                'id_categoria' => $juguetes->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1587654780291-39c9404d746b?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'JUGUET002',
                'nombre' => 'Rompecabezas 1000 Piezas',
                'detalles' => 'Paisaje panorámico, alta calidad',
                'descripcion' => 'Rompecabezas de 1000 piezas con imagen de paisaje panorámico, acabado mate de alta calidad.',
                'precio' => 24.99,
                'id_categoria' => $juguetes->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1543857778-c4a1a3e0b2eb?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],

            // Automotriz
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'AUTO001',
                'nombre' => 'Alfombrillas Universales',
                'detalles' => 'Set de 4, goma resistente',
                'descripcion' => 'Set de 4 alfombrillas universales de goma resistente, fáciles de limpiar y antideslizantes.',
                'precio' => 39.99,
                'id_categoria' => $auto->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1449965408869-eaa3f722e40d?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'AUTO002',
                'nombre' => 'Cámara de Reversa HD',
                'detalles' => 'Visión nocturna, resistente al agua',
                'descripcion' => 'Cámara de reversa en alta definición con visión nocturna y resistente al agua IP68.',
                'precio' => 54.99,
                'id_categoria' => $auto->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1492144534655-ae79c964c9d7?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],

            // Salud y Belleza
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'SALUD001',
                'nombre' => 'Báscula Digital Inteligente',
                'detalles' => 'Bluetooth, análisis corporal',
                'descripcion' => 'Báscula digital con conexión Bluetooth y análisis de composición corporal completo.',
                'precio' => 49.99,
                'id_categoria' => $salud->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1576671081837-49000212a370?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'SALUD002',
                'nombre' => 'Set de Cuidado Facial',
                'detalles' => 'Limpiador, tónico y crema',
                'descripcion' => 'Set completo de cuidado facial con limpiador, tónico y crema hidratante para todo tipo de piel.',
                'precio' => 39.99,
                'id_categoria' => $salud->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1556228578-8c89e6adf883?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'id' => Str::uuid()->toString(),
                'codigo' => 'SALUD003',
                'nombre' => 'Termómetro Digital Infrarrojo',
                'detalles' => 'Sin contacto, lectura rápida',
                'descripcion' => 'Termómetro infrarrojo sin contacto con lectura en 1 segundo y memoria de 32 mediciones.',
                'precio' => 34.99,
                'id_categoria' => $salud->id,
                'imagen_url' => 'https://images.unsplash.com/photo-1584515933487-779824d29309?w=500',
                'estado' => true,
                'created_at' => now(),
                'updated_at' => now(),
            ],
        ];

        foreach ($products as $product) {
            Product::create($product);
        }

        $this->command->info('Products seeded successfully!');
    }
}
