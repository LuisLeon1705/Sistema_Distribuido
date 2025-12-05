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