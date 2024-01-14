use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::{RequestValidator, TenantRequest, TypeInfo};
use crate::request::{RequestValidatorError, RequestValidatorResult};
use utoipa::ToSchema;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct ApplicationRequest {
    name: Option<String>,
    tenant: Option<TenantRequest>,
    class_unit: Option<String>,
    functional_group: Option<String>,
}

impl ApplicationRequest {
    pub fn new(n: Option<String>, t: Option<TenantRequest>, cu: Option<String>, fg: Option<String>) -> Self {
        Self {
            name: n,
            tenant: t,
            class_unit: cu,
            functional_group: fg,
        }
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn tenant(&self) -> Option<&TenantRequest> {
        self.tenant.as_ref()
    }

    pub fn class_unit(&self) -> Option<&String> {
        self.class_unit.as_ref()
    }

    pub fn functional_group(&self) -> Option<&String> {
        self.functional_group.as_ref()
    }

    pub fn build_get_request(name: String, tenant: String) -> Self {
        Self {
            name: Some(name),
            tenant: Some(TenantRequest::build_get_request(tenant)),
            class_unit: None,
            functional_group: None,
        }
    }
}

impl RequestValidator for ApplicationRequest {
    fn is_valid_create(&self) -> RequestValidatorResult {
        match (
            self.name.as_ref(),
            self.tenant.as_ref(),
            self.class_unit.as_ref(),
            self.functional_group.as_ref()
        ) {
            (Some(_), Some(tenant), Some(_), Some(_)) => tenant.is_valid_get(),
            (_, _, _, _) => Err(RequestValidatorError::MissingAttribute(format!("{} all attributes are mandatory", self.type_of()))),
        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        match (
            self.name.as_ref(),
            self.tenant.as_ref()
        ) {
            (Some(_), Some(tenant)) => tenant.is_valid_get(),
            (_, _) => Err(RequestValidatorError::MissingAttribute(format!("{} name and tenant are mandatory", self.type_of()))),
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.name.as_ref(),
            self.tenant.as_ref(),
            self.class_unit.as_ref(),
            self.functional_group.as_ref()
        ) {
            (None, None, None, None) => Err(RequestValidatorError::MissingAttribute(format!("{} at least one attribute is mandatory", self.type_of()))),
            (_, Some(tenant), _, _) => tenant.is_valid_get(),
            (_, None, _, _) => Ok(())
        }
    }
}

const APPLICATION_REQUEST_TYPE_INFO: &str = "ApplicationRequest";

impl TypeInfo for ApplicationRequest {
    fn type_name() -> &'static str {
        APPLICATION_REQUEST_TYPE_INFO
    }

    fn type_of(&self) -> &'static str {
        APPLICATION_REQUEST_TYPE_INFO
    }
}

impl FromStr for ApplicationRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: ApplicationRequest =
            serde_json::from_str(s).map_err(|e| format!("error parsing application request: {}", e))?;
        Ok(res)
    }
}
