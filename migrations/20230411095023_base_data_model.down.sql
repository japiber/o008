-- Add down migration script here

DROP TRIGGER audit_trigger_row on release_build_stage;
DROP TRIGGER audit_trigger_stm on release_build_stage;
DROP TABLE IF EXISTS release_build_stage;

DROP TRIGGER audit_trigger_row on release_build;
DROP TRIGGER audit_trigger_stm on release_build;
DROP TABLE IF EXISTS release_build;

DROP TRIGGER audit_trigger_row on release;
DROP TRIGGER audit_trigger_stm on release;
DROP TABLE IF EXISTS release;

DROP TRIGGER audit_trigger_row on service;
DROP TRIGGER audit_trigger_stm on service;
DROP TABLE IF EXISTS service;

DROP TRIGGER audit_trigger_row on builder;
DROP TRIGGER audit_trigger_stm on builder;
DROP TABLE IF EXISTS builder;

DROP TRIGGER audit_trigger_row on application;
DROP TRIGGER audit_trigger_stm on application;
DROP TABLE IF EXISTS application;

DROP TRIGGER audit_trigger_row on tenant;
DROP TRIGGER audit_trigger_stm on tenant;
DROP TABLE IF EXISTS tenant;