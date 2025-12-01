--
-- PostgreSQL database dump
--

\restrict fYxmfTfdcJI5QfZkhAgbIlXmcB7srlgq9aexMANgCyQvtMt9teO0VhngzHCZxmj

-- Dumped from database version 15.15
-- Dumped by pg_dump version 18.1

-- Started on 2025-12-01 13:23:11

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 219 (class 1259 OID 24713)
-- Name: cache; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.cache (
    key character varying(255) NOT NULL,
    value text NOT NULL,
    expiration integer NOT NULL
);


ALTER TABLE public.cache OWNER TO "user";

--
-- TOC entry 220 (class 1259 OID 24720)
-- Name: cache_locks; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.cache_locks (
    key character varying(255) NOT NULL,
    owner character varying(255) NOT NULL,
    expiration integer NOT NULL
);


ALTER TABLE public.cache_locks OWNER TO "user";

--
-- TOC entry 217 (class 1259 OID 24688)
-- Name: categorias; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.categorias (
    id bigint NOT NULL,
    nombre character varying(255) NOT NULL,
    descripcion text,
    prefijo_codigo character varying(255) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.categorias OWNER TO "user";

--
-- TOC entry 216 (class 1259 OID 24687)
-- Name: categorias_id_seq; Type: SEQUENCE; Schema: public; Owner: user
--

CREATE SEQUENCE public.categorias_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.categorias_id_seq OWNER TO "user";

--
-- TOC entry 3450 (class 0 OID 0)
-- Dependencies: 216
-- Name: categorias_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: user
--

ALTER SEQUENCE public.categorias_id_seq OWNED BY public.categorias.id;


--
-- TOC entry 215 (class 1259 OID 24681)
-- Name: migrations; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.migrations (
    id integer NOT NULL,
    migration character varying(255) NOT NULL,
    batch integer NOT NULL
);


ALTER TABLE public.migrations OWNER TO "user";

--
-- TOC entry 214 (class 1259 OID 24680)
-- Name: migrations_id_seq; Type: SEQUENCE; Schema: public; Owner: user
--

CREATE SEQUENCE public.migrations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.migrations_id_seq OWNER TO "user";

--
-- TOC entry 3451 (class 0 OID 0)
-- Dependencies: 214
-- Name: migrations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: user
--

ALTER SEQUENCE public.migrations_id_seq OWNED BY public.migrations.id;


--
-- TOC entry 218 (class 1259 OID 24698)
-- Name: productos; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.productos (
    id uuid NOT NULL,
    codigo character varying(255) NOT NULL,
    nombre character varying(255) NOT NULL,
    precio numeric(10,2) NOT NULL,
    id_categoria bigint NOT NULL,
    descripcion text NOT NULL,
    imagen_url character varying(255),
    estado boolean DEFAULT true NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.productos OWNER TO "user";

--
-- TOC entry 221 (class 1259 OID 24727)
-- Name: sessions; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.sessions (
    id character varying(255) NOT NULL,
    user_id bigint,
    ip_address character varying(45),
    user_agent text,
    payload text NOT NULL,
    last_activity integer NOT NULL
);


ALTER TABLE public.sessions OWNER TO "user";

--
-- TOC entry 3282 (class 2604 OID 24691)
-- Name: categorias id; Type: DEFAULT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.categorias ALTER COLUMN id SET DEFAULT nextval('public.categorias_id_seq'::regclass);


--
-- TOC entry 3281 (class 2604 OID 24684)
-- Name: migrations id; Type: DEFAULT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.migrations ALTER COLUMN id SET DEFAULT nextval('public.migrations_id_seq'::regclass);


--
-- TOC entry 3297 (class 2606 OID 24726)
-- Name: cache_locks cache_locks_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.cache_locks
    ADD CONSTRAINT cache_locks_pkey PRIMARY KEY (key);


--
-- TOC entry 3295 (class 2606 OID 24719)
-- Name: cache cache_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.cache
    ADD CONSTRAINT cache_pkey PRIMARY KEY (key);


--
-- TOC entry 3287 (class 2606 OID 24695)
-- Name: categorias categorias_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.categorias
    ADD CONSTRAINT categorias_pkey PRIMARY KEY (id);


--
-- TOC entry 3289 (class 2606 OID 24697)
-- Name: categorias categorias_prefijo_codigo_unique; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.categorias
    ADD CONSTRAINT categorias_prefijo_codigo_unique UNIQUE (prefijo_codigo);


--
-- TOC entry 3285 (class 2606 OID 24686)
-- Name: migrations migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.migrations
    ADD CONSTRAINT migrations_pkey PRIMARY KEY (id);


--
-- TOC entry 3291 (class 2606 OID 24712)
-- Name: productos productos_codigo_unique; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.productos
    ADD CONSTRAINT productos_codigo_unique UNIQUE (codigo);


--
-- TOC entry 3293 (class 2606 OID 24710)
-- Name: productos productos_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.productos
    ADD CONSTRAINT productos_pkey PRIMARY KEY (id);


--
-- TOC entry 3300 (class 2606 OID 24733)
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- TOC entry 3298 (class 1259 OID 24735)
-- Name: sessions_last_activity_index; Type: INDEX; Schema: public; Owner: user
--

CREATE INDEX sessions_last_activity_index ON public.sessions USING btree (last_activity);


--
-- TOC entry 3301 (class 1259 OID 24734)
-- Name: sessions_user_id_index; Type: INDEX; Schema: public; Owner: user
--

CREATE INDEX sessions_user_id_index ON public.sessions USING btree (user_id);


--
-- TOC entry 3302 (class 2606 OID 24704)
-- Name: productos productos_id_categoria_foreign; Type: FK CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.productos
    ADD CONSTRAINT productos_id_categoria_foreign FOREIGN KEY (id_categoria) REFERENCES public.categorias(id) ON DELETE CASCADE;


-- Completed on 2025-12-01 13:23:11

--
-- PostgreSQL database dump complete
--

\unrestrict fYxmfTfdcJI5QfZkhAgbIlXmcB7srlgq9aexMANgCyQvtMt9teO0VhngzHCZxmj

