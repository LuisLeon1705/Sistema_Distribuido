CREATE TABLE stock (
    id SERIAL PRIMARY KEY,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    warehouse_location VARCHAR(100)
);

INSERT INTO stock (id, product_id, quantity, last_updated, warehouse_location) VALUES
(5001, 101, 15, '2025-11-21T08:00:00Z', 'Pasillo A-12'),
(5002, 102, 120, '2025-11-20T18:45:00Z', 'Pasillo B-04'),
(5003, 103, 0, '2025-11-19T12:00:00Z', 'Pasillo C-01');

CREATE TYPE order_status AS ENUM ('pending', 'completed', 'cancelled');

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    total_price NUMERIC(10, 2) NOT NULL,
    status order_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INT REFERENCES orders(id) ON DELETE CASCADE,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    price_at_time_of_purchase NUMERIC(10, 2) NOT NULL
);

INSERT INTO orders (id, user_id, total_price, status, created_at) VALUES
(1, 2, 1246.49, 'completed', '2025-11-22T10:30:00Z');

INSERT INTO order_items (order_id, product_id, quantity, price_at_time_of_purchase) VALUES
(1, 101, 1, 1200.50),
(1, 102, 1, 45.99);