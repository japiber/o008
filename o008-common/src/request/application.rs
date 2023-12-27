use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::{RequestValidator, TenantRequest};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Application {
    name: Option<String>,
    tenant: Option<TenantRequest>,
    class_unit: Option<String>,
    functional_group: Option<String>,
}

impl Application {
    pub fn name(&self) -> &str {
        self.name.as_ref().unwrap().as_str()
    }

    pub fn tenant(&self) -> TenantRequest {
        self.tenant.as_ref().unwrap().clone()
    }

    pub fn class_unit(&self) -> &str {
        self.class_unit.as_ref().unwrap().as_str()
    }

    pub fn functional_group(&self) -> &str {
        self.functional_group.as_ref().unwrap().as_str()
    }
}

impl RequestValidator for Application {
    fn is_valid_create(&self) -> bool {
        self.name.is_some() && self.tenant.is_some() && self.class_unit.is_some() && self.functional_group.is_some()
    }

    fn is_valid_get(&self) -> bool {
        self.name.is_some() && self.tenant.is_some()
    }
}

impl FromStr for Application {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Application =
            serde_json::from_str(s).map_err(|e| format!("error parsing application request: {}", e))?;
        Ok(res)
    }
}
