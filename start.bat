@echo off
echo 🚀 Sistema Distribuido - Iniciando aplicación completa...
echo.

if "%1"=="build" goto build
if "%1"=="up" goto up
if "%1"=="down" goto down
if "%1"=="restart" goto restart
if "%1"=="logs" goto logs
if "%1"=="status" goto status
if "%1"=="clean" goto clean
if "%1"=="help" goto help
if "%1"=="-h" goto help
if "%1"=="--help" goto help
if "%1"=="" goto default
goto invalid

:help
echo Uso: start.bat [OPCIÓN]
echo.
echo Opciones:
echo   build     Construir todas las imágenes Docker
echo   up        Iniciar todos los servicios
echo   down      Detener todos los servicios
echo   restart   Reiniciar todos los servicios
echo   logs      Mostrar logs de todos los servicios
echo   status    Mostrar estado de los servicios
echo   clean     Limpiar imágenes y contenedores
echo   help      Mostrar esta ayuda
echo.
echo Sin opciones: Ejecuta build + up
goto end

:build
echo 🔨 Construyendo imágenes Docker...
docker-compose build --no-cache
goto end

:up
echo ▶️  Iniciando servicios...
docker-compose up -d
echo.
echo ✅ Sistema iniciado correctamente!
echo.
echo 🌐 URLs disponibles:
echo   Frontend:         http://localhost:3000
echo   AuthService:      http://localhost:8001
echo   InventoryService: http://localhost:8002
echo   ProductService:   http://localhost:8003
echo   PostgreSQL:       localhost:5432
echo.
echo 📋 Para ver logs: docker-compose logs -f
echo 🛑 Para detener:  start.bat down
goto end

:down
echo ⏹️  Deteniendo servicios...
docker-compose down
echo ✅ Servicios detenidos
goto end

:restart
echo ⏹️  Reiniciando servicios...
docker-compose down
echo 🔨 Construyendo imágenes Docker...
docker-compose build --no-cache
echo ▶️  Iniciando servicios...
docker-compose up -d
echo ✅ Sistema reiniciado correctamente!
goto end

:logs
echo 📋 Mostrando logs (Ctrl+C para salir)...
docker-compose logs -f
goto end

:status
echo 📊 Estado de los servicios:
docker-compose ps
goto end

:clean
echo 🧹 Limpiando sistema...
docker-compose down -v
docker system prune -f
echo ✅ Sistema limpiado
goto end

:default
call :build
call :up
goto end

:invalid
echo ❌ Opción no válida: %1
echo.
goto help

:end
