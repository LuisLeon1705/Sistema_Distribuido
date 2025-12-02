
CREATE TABLE public.cache (
    key character varying(255) NOT NULL,
    value text NOT NULL,
    expiration integer NOT NULL
);


ALTER TABLE public.cache OWNER TO "user";

--
-- TOC entry 219 (class 1259 OID 25187)
-- Name: cache_locks; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.cache_locks (
    key character varying(255) NOT NULL,
    owner character varying(255) NOT NULL,
    expiration integer NOT NULL
);


ALTER TABLE public.cache_locks OWNER TO "user";

--
-- TOC entry 216 (class 1259 OID 25156)
-- Name: categorias; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.categorias (
    id uuid NOT NULL,
    nombre character varying(255) NOT NULL,
    descripcion text,
    codigo character varying(255) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.categorias OWNER TO "user";

--
-- TOC entry 215 (class 1259 OID 25150)
-- Name: migrations; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.migrations (
    id integer NOT NULL,
    migration character varying(255) NOT NULL,
    batch integer NOT NULL
);


ALTER TABLE public.migrations OWNER TO "user";

--
-- TOC entry 214 (class 1259 OID 25149)
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
-- TOC entry 3448 (class 0 OID 0)
-- Dependencies: 214
-- Name: migrations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: user
--

ALTER SEQUENCE public.migrations_id_seq OWNED BY public.migrations.id;


--
-- TOC entry 217 (class 1259 OID 25165)
-- Name: productos; Type: TABLE; Schema: public; Owner: user
--

CREATE TABLE public.productos (
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
    updated_at timestamp(0) without time zone
);


ALTER TABLE public.productos OWNER TO "user";

--
-- TOC entry 220 (class 1259 OID 25194)
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
-- TOC entry 3280 (class 2604 OID 25153)
-- Name: migrations id; Type: DEFAULT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.migrations ALTER COLUMN id SET DEFAULT nextval('public.migrations_id_seq'::regclass);


--
-- TOC entry 3295 (class 2606 OID 25193)
-- Name: cache_locks cache_locks_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.cache_locks
    ADD CONSTRAINT cache_locks_pkey PRIMARY KEY (key);


--
-- TOC entry 3293 (class 2606 OID 25186)
-- Name: cache cache_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.cache
    ADD CONSTRAINT cache_pkey PRIMARY KEY (key);


--
-- TOC entry 3285 (class 2606 OID 25164)
-- Name: categorias categorias_codigo_unique; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.categorias
    ADD CONSTRAINT categorias_codigo_unique UNIQUE (codigo);


--
-- TOC entry 3287 (class 2606 OID 25162)
-- Name: categorias categorias_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.categorias
    ADD CONSTRAINT categorias_pkey PRIMARY KEY (id);


--
-- TOC entry 3283 (class 2606 OID 25155)
-- Name: migrations migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.migrations
    ADD CONSTRAINT migrations_pkey PRIMARY KEY (id);


--
-- TOC entry 3289 (class 2606 OID 25179)
-- Name: productos productos_codigo_unique; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.productos
    ADD CONSTRAINT productos_codigo_unique UNIQUE (codigo);


--
-- TOC entry 3291 (class 2606 OID 25177)
-- Name: productos productos_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.productos
    ADD CONSTRAINT productos_pkey PRIMARY KEY (id);


--
-- TOC entry 3298 (class 2606 OID 25200)
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- TOC entry 3296 (class 1259 OID 25202)
-- Name: sessions_last_activity_index; Type: INDEX; Schema: public; Owner: user
--

CREATE INDEX sessions_last_activity_index ON public.sessions USING btree (last_activity);


--
-- TOC entry 3299 (class 1259 OID 25201)
-- Name: sessions_user_id_index; Type: INDEX; Schema: public; Owner: user
--

CREATE INDEX sessions_user_id_index ON public.sessions USING btree (user_id);


--
-- TOC entry 3300 (class 2606 OID 25171)
-- Name: productos productos_id_categoria_foreign; Type: FK CONSTRAINT; Schema: public; Owner: user
--

ALTER TABLE ONLY public.productos
    ADD CONSTRAINT productos_id_categoria_foreign FOREIGN KEY (id_categoria) REFERENCES public.categorias(id) ON DELETE CASCADE;


