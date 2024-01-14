use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Postgres;
use uuid::Uuid;
use crate::{CommandContext, DalError, DaoCommand, DaoQuery, QueryContext, DalCount, gen_v7_uuid};
use crate::pg::{hard_check_key, PgDao, Tenant};


#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Application {
    id: Uuid,
    name: String,
    tenant: Uuid,
    class_unit: String,
    functional_group: String,
}

impl Application {
    pub fn new(id: Uuid, name: &str, tenant: Uuid, class: &str, fq: &str) -> Self {
        Self {
            id: gen_v7_uuid(id),
            name: String::from(name),
            tenant,
            class_unit: String::from(class),
            functional_group: String::from(fq),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tenant(&self) -> Uuid {
        self.tenant
    }

    pub fn class_unit(&self) -> &str {
        &self.class_unit
    }

    pub fn functional_group(&self) -> &str {
        &self.functional_group
    }
}

#[async_trait]
impl DaoQuery<PgDao, Postgres> for Application {
    async fn read(key: Value) -> Result<Box<Self>, DalError> {
        match hard_check_key(&key, &["id"]) {
            Ok(id_key) => {
                let id= id_key.first().unwrap();
                Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, Self>("SELECT id, name, tenant, class_unit, functional_group FROM application WHERE id=$1")
                        .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
                ).await
            },
            Err(_) => match hard_check_key(&key, &["name", "tenant"]) {
                Ok(name_tenant_key) => {
                    let name = name_tenant_key.get(0).unwrap().as_str().unwrap();
                    let tenant_qry = name_tenant_key.get(1).unwrap();
                    if let Ok(tenant) = Tenant::read(tenant_qry.clone()).await {
                        Self::query_ctx().await.fetch_one(
                            sqlx::query_as::<_, Self>("SELECT id, name, tenant, class_unit, functional_group FROM application WHERE name=$1 AND tenant=$2")
                                .bind(name)
                                .bind(tenant.id())
                        ).await
                    } else {
                        Err(DalError::DataNotFound(format!("tenant {}", tenant_qry)))
                    }
                },
                Err(e) => Err(DalError::InvalidKey(format!("application dao read {}", e)))
            }
        }
    }

    async fn exists(key: Value) -> bool {
        if let Ok(id_key) = hard_check_key(&key, &["id"]) {
            let id = id_key.first().unwrap();
            let r = Self::query_ctx().await.fetch_one(
                sqlx::query_as::<_, DalCount>("SELECT COUNT(*) AS count FROM application WHERE id=$1")
                    .bind(Uuid::parse_str(id.as_str().unwrap()).unwrap())
            ).await;
            r.unwrap().count > 0
        } else if let Ok(name_tenant_key) = hard_check_key(&key, &["name", "tenant"]) {
            let (name, tenant_qry) = (name_tenant_key.get(0).unwrap(), name_tenant_key.get(1).unwrap());
            if let Ok(tenant) = Tenant::read(tenant_qry.clone()).await {
                let r = Self::query_ctx().await.fetch_one(
                    sqlx::query_as::<_, DalCount>("SELECT COUNT(*) WHERE name=$1 AND tenant=$2")
                        .bind(name.as_str().unwrap())
                        .bind(tenant.id())
                ).await;
                r.unwrap().count > 0
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[async_trait]
impl DaoCommand<PgDao, Postgres> for Application {
    async fn insert(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("INSERT INTO application(id, name, tenant, class_unit, functional_group) VALUES ($1, $2, $3, $4, $5)")
                .bind(self.id)
                .bind(self.name.as_str())
                .bind(self.tenant)
                .bind(self.class_unit.as_str())
                .bind(self.functional_group.as_str())
        ).await
    }

    async fn update(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("UPDATE application SET name=$1, tenant=$2, class_unit=$3, functional_group=$4 WHERE id=$5")
                .bind(self.name.as_str())
                .bind(self.tenant)
                .bind(self.class_unit.as_str())
                .bind(self.functional_group.as_str())
                .bind(self.id)
        ).await
    }

    async fn delete(&self) -> Result<(), DalError> {
        let cx = Self::command_ctx().await;
        cx.execute(
            sqlx::query("DELETE FROM application WHERE id = $1")
                .bind(self.id)
        ).await
    }
}
