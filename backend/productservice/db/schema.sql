CREATE TABLE cache (
    key character varying(255) NOT NULL,
    value text NOT NULL,
    expiration integer NOT NULL,
    CONSTRAINT cache_pkey PRIMARY KEY (key)
);

CREATE TABLE cache_locks (
    key character varying(255) NOT NULL,
    owner character varying(255) NOT NULL,
    expiration integer NOT NULL,
    CONSTRAINT cache_locks_pkey PRIMARY KEY (key)
);

CREATE TABLE sessions (
    id character varying(255) NOT NULL,
    user_id bigint,
    ip_address character varying(45),
    user_agent text,
    payload text NOT NULL,
    last_activity integer NOT NULL,
    CONSTRAINT sessions_pkey PRIMARY KEY (id)
);

CREATE INDEX sessions_last_activity_index ON sessions (last_activity);
CREATE INDEX sessions_user_id_index ON sessions (user_id);

CREATE SEQUENCE migrations_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE migrations (
    id integer NOT NULL DEFAULT nextval('migrations_id_seq'),
    migration character varying(255) NOT NULL,
    batch integer NOT NULL,
    CONSTRAINT migrations_pkey PRIMARY KEY (id)
);

ALTER SEQUENCE migrations_id_seq OWNED BY migrations.id;

CREATE TABLE categorias (
    id uuid NOT NULL,
    nombre character varying(255) NOT NULL,
    descripcion text,
    codigo character varying(255) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    
    -- Restricciones
    CONSTRAINT categorias_pkey PRIMARY KEY (id),
    CONSTRAINT categorias_codigo_unique UNIQUE (codigo)
);

CREATE TABLE productos (
    id uuid NOT NULL,
    codigo character varying(255) NOT NULL,
    nombre character varying(255) NOT NULL,
    detalles character varying(255),
    precio numeric(10,2) NOT NULL,
    id_categoria uuid NOT NULL,
    descripcion text NOT NULL,
    imagen_url character varying(255),
    estado boolean DEFAULT true NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    
    CONSTRAINT productos_pkey PRIMARY KEY (id),
    CONSTRAINT productos_codigo_unique UNIQUE (codigo),
    
    CONSTRAINT productos_id_categoria_foreign 
        FOREIGN KEY (id_categoria) 
        REFERENCES categorias(id) 
        ON DELETE CASCADE
);

INSERT INTO categorias (id, nombre, descripcion, codigo, created_at, updated_at) VALUES
('11111111-1111-1111-1111-111111111111', 'Bebidas', 'Bebidas para cada ocasión: jugos, refrescos y más.', 'BEB', NOW(), NOW()),
('22222222-2222-2222-2222-222222222222', 'Víveres', 'Productos básicos de tu despensa.', 'VIB', NOW(), NOW()),
('33333333-3333-3333-3333-333333333333', 'Dulces', 'Endulza tus momentos con caramelos, chocolates y golosinas irresistibles.', 'DUL', NOW(), NOW());

INSERT INTO productos (id, codigo, nombre, detalles, precio, id_categoria, descripcion, imagen_url, estado, created_at, updated_at) VALUES
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'BEB-000001', 'Coca Cola Original', '2 litros', 3.00, '11111111-1111-1111-1111-111111111111', 'Bebida gaseosa clásica', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/cocacola.jpg', true, NOW(), NOW()),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'BEB-000002', 'Jugo de Naranja', '1 litro', 2.50, '11111111-1111-1111-1111-111111111111', 'Saludable y natrual', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/naranja.jpg', true, NOW(), NOW()),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', 'BEB-000003', 'Te de Manzana', '750 ml', 5.00, '11111111-1111-1111-1111-111111111111', 'Refrescante y delicioso', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/manzana.jpg', true, NOW(), NOW()),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14', 'BEB-000004', 'Arizona', '350 ml', 1.25, '11111111-1111-1111-1111-111111111111', 'Recarga energías con Arizona', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/arizona.png', true, NOW(), NOW()),

('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380b21', 'VIB-000001', 'Harina', '1 kl', 1.10, '22222222-2222-2222-2222-222222222222', 'Ideal para repostería', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/harina.jpg', true, NOW(), NOW()),
('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380b22', 'VIB-000002', 'Pasta', '1 kl', 2.20, '22222222-2222-2222-2222-222222222222', 'Fácil de cocinar', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/pasta.jpg', true, NOW(), NOW()),
('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380b23', 'VIB-000003', 'Aceite', '750 ml', 4.35, '22222222-2222-2222-2222-222222222222', 'Oliva extra virgen', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/aceite.jpg', true, NOW(), NOW()),
('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380b24', 'VIB-000004', 'Arroz', '1 kl', 1.90, '22222222-2222-2222-2222-222222222222', 'Grano largo premium', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/descarga.jpg', true, NOW(), NOW()),

('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380c31', 'DUL-000001', 'Galleta Oreo', '150 mg', 2.50, '33333333-3333-3333-3333-333333333333', 'Clásica y deliciosa', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/oreo.jpg', true, NOW(), NOW()),
('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380c32', 'DUL-000002', 'Chocolate Savoy', '350 mg', 3.60, '33333333-3333-3333-3333-333333333333', 'Delicioso chocolate oscuro', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/chocolate.jpg', true, NOW(), NOW()),
('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380c33', 'DUL-000003', 'Pringles', '400 mg', 4.00, '33333333-3333-3333-3333-333333333333', 'Papas fritas en tubo', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/pringles.jpg', true, NOW(), NOW()),
('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380c34', 'DUL-000004', 'Flips de Chocolate', '150 mg', 2.30, '33333333-3333-3333-3333-333333333333', 'Crujientes y dulces', 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/flips.jpg', true, NOW(), NOW());