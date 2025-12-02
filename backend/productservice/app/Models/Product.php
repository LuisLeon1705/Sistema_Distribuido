<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Concerns\HasUuids;

class Product extends Model
{
    use HasFactory, HasUuids;   

    protected $table = 'productos';
    public $incrementing = false;
    protected $keyType = 'string';
    
    protected $fillable = [
        'codigo',
        'nombre',
        'precio',
        'id_categoria',
        'descripcion',
        'imagen_url',
        'estado'
    ];

    // Append compatibility attributes for frontend which uses
    // 'categoria_id', 'imagen' and expects 'estado' as 'activo'/'inactivo'
    protected $appends = ['categoria_id', 'imagen', 'estado'];

    public function categoria()
    {
        return $this->belongsTo(Category::class, 'id_categoria', 'id');
    }

    // Accessor for categoria_id to keep frontend compatibility
    public function getCategoriaIdAttribute()
    {
        return $this->attributes['id_categoria'] ?? null;
    }

    // Accessor that exposes imagen for the frontend, mapping to imagen_url
    public function getImagenAttribute()
    {
        return $this->attributes['imagen_url'] ?? null;
    }

    // Accessor to provide estado as a friendly text value for frontend
    public function getEstadoAttribute($value)
    {
        if (!array_key_exists('estado', $this->attributes)) return null;
        return $this->attributes['estado'] ? 'activo' : 'inactivo';
    }
}
