CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price NUMERIC(10, 2) NOT NULL,
    category VARCHAR(100),
    image_url VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO products (id, name, description, price, category, image_url, created_at) VALUES
(101, 'Laptop Gamer X1', 'Portátil de alto rendimiento con RTX 4060', 1200.50, 'Computación', NULL, '2025-10-01T09:00:00Z'),
(102, 'Mouse Inalámbrico Pro', 'Mouse ergonómico con batería de larga duración', 45.99, 'Accesorios', NULL, '2025-10-05T11:15:00Z'),
(103, 'Monitor 4K Ultra', 'Pantalla IPS de 27 pulgadas', 350.00, 'Monitores', NULL, '2025-10-06T16:20:00Z');
