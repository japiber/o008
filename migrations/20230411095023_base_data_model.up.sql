-- Add up migration script here

CREATE TABLE IF NOT EXISTS builder
(
    id            uuid              NOT NULL,
    name          character varying NOT NULL,
    active        boolean           NOT NULL,
    build_command character varying NOT NULL,
    CONSTRAINT    builder_pkey PRIMARY KEY (id),
    CONSTRAINT    builder_name_key UNIQUE (name)
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
    CONSTRAINT   service_pkey PRIMARY KEY (id),
    CONSTRAINT   service_name_key UNIQUE (name, application),
    CONSTRAINT   service_application_fkey FOREIGN KEY (application) REFERENCES application (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.service');

CREATE TABLE IF NOT EXISTS repo_reference
(
    id         uuid              NOT NULL,
    repo       character varying NOT NULL,
    kind       character varying NOT NULL,
    reference  character varying NOT NULL,
    CONSTRAINT repo_reference_pkey PRIMARY KEY (id)
);

SELECT audit.audit_table('public.repo_reference');

CREATE TABLE IF NOT EXISTS service_version
(
    id         uuid              NOT NULL,
    version    character varying NOT NULL,
    is_release boolean           NOT NULL,
    service    uuid              NOT NULL,
    repo_ref   uuid              NOT NULL,
    builder    uuid              NOT NULL,
    CONSTRAINT service_version_pkey PRIMARY KEY (id),
    CONSTRAINT service_version_service_key UNIQUE (version, service),
    CONSTRAINT service_version_builder_fkey FOREIGN KEY (builder) REFERENCES builder (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT service_version_service_fkey FOREIGN KEY (service) REFERENCES service (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT service_version_repo_ref_fkey FOREIGN KEY (repo_ref) REFERENCES repo_reference (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.service_version');

CREATE TABLE IF NOT EXISTS service_version_build
(
    id              uuid              NOT NULL,
    service_version uuid              NOT NULL,
    status          character varying NOT NULL,
    completed       boolean           NOT NULL,
    in_error        boolean           NOT NULL,
    start_on        timestamp with time zone,
    end_on          timestamp with time zone,
    CONSTRAINT      service_version_build_pkey PRIMARY KEY (id),
    CONSTRAINT      service_version_build_service_version_fkey FOREIGN KEY (service_version) REFERENCES service_version (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.service_version_build');

CREATE TABLE IF NOT EXISTS service_version_build_stage
(
    id           uuid                     NOT NULL,
    build        uuid                     NOT NULL,
    stage        integer                  NOT NULL,
    name         character varying        NOT NULL,
    description  character varying,
    stage_meta   jsonb,
    status       character varying,
    completed    boolean                  NOT NULL,
    in_error     boolean                  NOT NULL,
    error_reason character varying,
    logs_link    character varying,
    start_on     timestamp with time zone NOT NULL,
    end_on       timestamp with time zone,
    CONSTRAINT   service_version_build_stage_pkey PRIMARY KEY (id),
    CONSTRAINT   service_version_build_stage_build_sequence_key UNIQUE (build, stage),
    CONSTRAINT   service_version_build_stage_build_fkey FOREIGN KEY (build) REFERENCES service_version_build (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
);

SELECT audit.audit_table('public.service_version_build_stage');
