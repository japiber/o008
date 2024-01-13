use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::request::{RequestValidatorError, RequestValidatorResult};
use crate::{RequestValidator, TypeInfo};
use utoipa::ToSchema;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct TenantRequest {
    pub name: Option<String>,
    pub coexisting: Option<bool>
}

impl TenantRequest {
    pub fn new(name: Option<String>, coexisting: Option<bool>) -> Self {
        Self {
            name,
            coexisting,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref().unwrap().as_str()
    }
    
    pub fn coexisting(&self) -> bool {
        *self.coexisting.as_ref().unwrap()
    }

    pub fn build_get_request(name: String) -> Self {
        Self {
            name: Some(name),
            coexisting: None
        }
    }
}

impl RequestValidator for TenantRequest {
    fn is_valid_create(&self) -> RequestValidatorResult {
        match (
            self.name.as_ref(),
            self.coexisting
        ) {
            (Some(_), Some(_)) => Ok(()),
            (_, _) => Err(RequestValidatorError::MissingAttribute(format!("{} all attributes are mandatory", self.type_of()))),

        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        if let Some(_) = self.name.as_ref() {
            Ok(())
        } else {
            Err(RequestValidatorError::MissingAttribute(format!("{} name attribute is mandatory", self.type_of())))
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.name.as_ref(),
            self.coexisting
        ) {
            (Some(_), Some(_)) => Ok(()),
            (_, _) => Err(RequestValidatorError::MissingAttribute(format!("{} at least one attribute is mandatory", self.type_of()))),
        }
    }
}

const TENANT_REQUEST_TYPE_INFO: &str = "TenantRequest";

impl TypeInfo for TenantRequest {
    fn type_name() -> &'static str {
        TENANT_REQUEST_TYPE_INFO
    }

    fn type_of(&self) -> &'static str {
        TENANT_REQUEST_TYPE_INFO
    }
}

impl FromStr for TenantRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: TenantRequest =
            serde_json::from_str(s).map_err(|e| format!("error parsing tenant request: {}", e))?;
        Ok(res)
    }
}
