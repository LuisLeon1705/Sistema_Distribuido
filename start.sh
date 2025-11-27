#!/bin/bash

echo "🚀 Sistema Distribuido - Iniciando aplicación completa..."
echo ""

# Función para mostrar ayuda
show_help() {
    echo "Uso: ./start.sh [OPCIÓN]"
    echo ""
    echo "Opciones:"
    echo "  build     Construir todas las imágenes Docker"
    echo "  up        Iniciar todos los servicios"
    echo "  down      Detener todos los servicios"
    echo "  restart   Reiniciar todos los servicios"
    echo "  logs      Mostrar logs de todos los servicios"
    echo "  status    Mostrar estado de los servicios"
    echo "  clean     Limpiar imágenes y contenedores"
    echo "  help      Mostrar esta ayuda"
    echo ""
    echo "Sin opciones: Ejecuta build + up"
}

# Función para construir imágenes
build_images() {
    echo "🔨 Construyendo imágenes Docker..."
    docker-compose build --no-cache
}

# Función para iniciar servicios
start_services() {
    echo "▶️  Iniciando servicios..."
    docker-compose up -d
    
    echo ""
    echo "✅ Sistema iniciado correctamente!"
    echo ""
    echo "🌐 URLs disponibles:"
    echo "  Frontend:         http://localhost:3000"
    echo "  AuthService:      http://localhost:8001"
    echo "  InventoryService: http://localhost:8002"
    echo "  ProductService:   http://localhost:8003"
    echo "  PostgreSQL:       localhost:5432"
    echo ""
    echo "📋 Para ver logs: docker-compose logs -f"
    echo "🛑 Para detener:  ./start.sh down"
}

# Función para detener servicios
stop_services() {
    echo "⏹️  Deteniendo servicios..."
    docker-compose down
    echo "✅ Servicios detenidos"
}

# Función para mostrar logs
show_logs() {
    echo "📋 Mostrando logs (Ctrl+C para salir)..."
    docker-compose logs -f
}

# Función para mostrar status
show_status() {
    echo "📊 Estado de los servicios:"
    docker-compose ps
}

# Función para limpiar
clean_system() {
    echo "🧹 Limpiando sistema..."
    docker-compose down -v
    docker system prune -f
    echo "✅ Sistema limpiado"
}

# Procesar argumentos
case "$1" in
    "build")
        build_images
        ;;
    "up")
        start_services
        ;;
    "down")
        stop_services
        ;;
    "restart")
        stop_services
        build_images
        start_services
        ;;
    "logs")
        show_logs
        ;;
    "status")
        show_status
        ;;
    "clean")
        clean_system
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    "")
        build_images
        start_services
        ;;
    *)
        echo "❌ Opción no válida: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
