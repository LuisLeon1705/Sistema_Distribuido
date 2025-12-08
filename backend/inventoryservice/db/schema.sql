CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE stock (
    id SERIAL PRIMARY KEY,
    product_id UUID NOT NULL,
    quantity INT NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    warehouse_location VARCHAR(100)
);

CREATE TYPE order_status AS ENUM ('pending', 'completed', 'cancelled');

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    total_price NUMERIC(10, 2) NOT NULL,
    status order_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INT REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL,
    quantity INT NOT NULL,
    price_at_time_of_purchase NUMERIC(10, 2) NOT NULL
);
