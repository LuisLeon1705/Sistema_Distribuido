CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    rol VARCHAR(50) NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (id, username, email, password_hash, rol, verified, created_at) VALUES
(1, 'admin_user', 'admin@sistema.com', '$2b$12$eIxIsYg...', 'admin', TRUE, '2025-11-20T10:00:00Z'),
(2, 'cliente_juan', 'juan@gmail.com', '$2b$12$rFvMsQz...', 'customer', FALSE, '2025-11-21T14:30:00Z');
