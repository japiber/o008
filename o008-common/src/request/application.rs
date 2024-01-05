use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::{RequestValidator, TenantRequest};
use crate::request::{RequestValidatorError, RequestValidatorResult};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Application {
    name: Option<String>,
    tenant: Option<TenantRequest>,
    class_unit: Option<String>,
    functional_group: Option<String>,
}

impl Application {
    pub fn new(n: Option<String>, t: Option<TenantRequest>, cu: Option<String>, fg: Option<String>) -> Self {
        Self {
            name: n,
            tenant: t,
            class_unit: cu,
            functional_group: fg,
        }
    }

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
    fn is_valid_create(&self) -> RequestValidatorResult {
        match (
            self.name.is_some(),
            self.tenant.as_ref().is_some(),
            self.class_unit.is_some(),
            self.functional_group.is_some()
        ) {
            (false, _, _, _) => Err(RequestValidatorError::MissingAttribute("name".to_string())),
            (_, false, _, _) => Err(RequestValidatorError::MissingAttribute("tenant".to_string())),
            (_, _, false, _) => Err(RequestValidatorError::MissingAttribute("class_unit".to_string())),
            (_, _, _, false) => Err(RequestValidatorError::MissingAttribute("functional_group".to_string())),
            (true, true, true, true) => self.tenant.as_ref().unwrap().is_valid_get(),
        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        match (
            self.name.is_some(),
            self.tenant.as_ref().is_some()
        ) {
            (false, _) => Err(RequestValidatorError::MissingAttribute("name".to_string())),
            (_, false) => Err(RequestValidatorError::MissingAttribute("tenant".to_string())),
            (true, true) => self.tenant.as_ref().unwrap().is_valid_get(),
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.name.is_some(),
            self.tenant.as_ref().is_some(),
            self.class_unit.is_some(),
            self.functional_group.is_some()
        ) {
            (false, false, false, false) => Err(RequestValidatorError::AtLeastOneRequired),
            (_, true, _, _) => self.tenant.as_ref().unwrap().is_valid_get(),
            (_, _, _, _) => Ok(())
        }
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
