-- Add up migration script here

CREATE TABLE IF NOT EXISTS builder
(
    id      uuid              NOT NULL,
    name    character varying NOT NULL,
    active  boolean           NOT NULL,
    command character varying NOT NULL,
    CONSTRAINT builder_pkey PRIMARY KEY (id),
    CONSTRAINT builder_name_key UNIQUE (name)
);

SELECT audit.audit_table('public.builder');

CREATE TABLE IF NOT EXISTS tenant
(
    id         uuid              NOT NULL,
    name       character varying NOT NULL,
    coexisting boolean           NOT NULL,
    CONSTRAINT tenant_pkey PRIMARY KEY (id),
    CONSTRAINT tenant_name_key UNIQUE (name)
);

SELECT audit.audit_table('public.tenant');

CREATE TABLE IF NOT EXISTS application
(
    id         uuid              NOT NULL,
    name       character varying NOT NULL,
    tenant     uuid              NOT NULL,
    class_unit character varying NOT NULL,
    CONSTRAINT application_pkey PRIMARY KEY (id),
    CONSTRAINT application_name_tenant_key UNIQUE (name, tenant),
    CONSTRAINT application_tenant_fkey FOREIGN KEY (tenant) REFERENCES tenant (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.application');

CREATE TABLE IF NOT EXISTS service
(
    id           uuid              NOT NULL,
    name         character varying NOT NULL,
    application  uuid              NOT NULL,
    status       character varying,
    default_repo character varying NOT NULL,
    CONSTRAINT service_pkey PRIMARY KEY (id),
    CONSTRAINT service_name_key UNIQUE (name),
    CONSTRAINT service_application_fkey FOREIGN KEY (application) REFERENCES application (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.service');

CREATE TABLE IF NOT EXISTS release
(
    id        uuid              NOT NULL,
    version   character varying NOT NULL,
    service   uuid              NOT NULL,
    repo      character varying NOT NULL,
    commit_id character varying NOT NULL,
    builder   uuid              NOT NULL,
    CONSTRAINT release_pkey PRIMARY KEY (id),
    CONSTRAINT release_version_service_key UNIQUE (version, service),
    CONSTRAINT release_builder_fkey FOREIGN KEY (builder) REFERENCES builder (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT release_service_fkey FOREIGN KEY (service) REFERENCES service (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.release');

CREATE TABLE IF NOT EXISTS release_build
(
    id             uuid              NOT NULL,
    release        uuid              NOT NULL,
    status         character varying NOT NULL,
    completed      boolean           NOT NULL,
    in_error       boolean           NOT NULL,
    start_on       timestamp with time zone,
    execution_time bigint,
    CONSTRAINT release_build_pkey PRIMARY KEY (id),
    CONSTRAINT release_build_release_fkey FOREIGN KEY (release) REFERENCES release (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.release_build');

CREATE TABLE IF NOT EXISTS release_build_stage
(
    id           uuid                     NOT NULL,
    build        uuid                     NOT NULL,
    sequence     integer                  NOT NULL,
    stage        character varying        NOT NULL,
    description  character varying,
    stage_meta   jsonb,
    status       character varying,
    completed    boolean                  NOT NULL,
    in_error     boolean                  NOT NULL,
    error_reason character varying,
    logs_link    character varying,
    start_on     timestamp with time zone NOT NULL,
    end_on       timestamp with time zone,
    CONSTRAINT release_build_stage_pkey PRIMARY KEY (id),
    CONSTRAINT release_build_stage_sequence_key UNIQUE (build, sequence),
    CONSTRAINT release_build_fkey FOREIGN KEY (build) REFERENCES release_build (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.release_build_stage');
