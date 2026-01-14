CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    total DOUBLE PRECISION NOT NULL,
    status VARCHAR(255) NOT NULL,
    created_at TIMESTAMP
);
CREATE TABLE IF NOT EXISTS order_items (
    id UUID PRIMARY KEY,
    product_id UUID,
    quantity INTEGER,
    price DOUBLE PRECISION,
    order_id UUID,
    CONSTRAINT fk_order_items_orders
        FOREIGN KEY (order_id)
        REFERENCES orders (id)
        ON DELETE CASCADE
);