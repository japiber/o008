use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::request::{RequestValidatorError, RequestValidatorResult};
use crate::RequestValidator;
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
}

impl RequestValidator for TenantRequest {
    fn is_valid_create(&self) -> RequestValidatorResult {
        match (
            self.name.is_some(),
            self.coexisting.is_some()
        ) {
            (false, _) => Err(RequestValidatorError::MissingAttribute("name".to_string())),
            (_, false) => Err(RequestValidatorError::MissingAttribute("coexisting".to_string())),
            (true, true) => Ok(()),
        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        if self.name.is_some() {
            Ok(())
        } else {
            Err(RequestValidatorError::MissingAttribute("name".to_string()))
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.name.is_some(),
            self.coexisting.is_some()
        ) {
            (false, false) => Err(RequestValidatorError::AtLeastOneRequired),
            (_, _) => Ok(()),
        }
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
