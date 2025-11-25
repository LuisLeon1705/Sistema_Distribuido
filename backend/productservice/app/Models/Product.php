<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class Product extends Model
{
    use HasFactory;

    protected $table = 'productos';
    
    protected $fillable = [
        'id_categoria',
        'nombre',
        'descripcion',
        'precio',
        'imagen_url',
        'estado'
    ];

    public function categoria()
    {
        return $this->belongsTo(Category::class, 'id_categoria', 'id');
    }
}
