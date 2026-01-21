#!/bin/bash
set -e

# Perform all actions as the 'postgres' user
export PGUSER="$POSTGRES_USER"

# Create databases
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE DATABASE auth_db;
    CREATE DATABASE inventory_db;
    CREATE DATABASE products_db;
    CREATE DATABASE orders_db;
EOSQL

# Load schemas into their respective databases
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "auth_db" -f /schemas/auth_schema.sql
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "inventory_db" -f /schemas/inventory_schema.sql
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "products_db" -f /schemas/products_schema.sql
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "orders_db" -f /schemas/orders_schema.sql
