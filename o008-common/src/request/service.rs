use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::{ApplicationRequest, RequestValidator, TenantRequest};
use crate::request::{RequestValidatorError, RequestValidatorResult};
use utoipa::ToSchema;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct ServiceRequest {
    name: Option<String>,
    application: Option<ApplicationRequest>,
    default_repo: Option<String>,
}

impl ServiceRequest {
    pub fn new(n: Option<String>, app: Option<ApplicationRequest>, repo: Option<String>) -> Self {
        Self {
            name: n,
            application: app,
            default_repo: repo,
        }
    }

    pub fn build_get_request(name: String, application: String, tenant: String) -> Self {
        Self::new(
            Some(name),
                Some(
                    ApplicationRequest::new(
                        Some(application),
                        Some(TenantRequest::new(Some(tenant), None)),
                        None,
                        None
                    )
                ),
            None
        )
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn application(&self) -> Option<ApplicationRequest> {
        self.application.clone()
    }

    pub fn default_repo(&self) -> Option<String> {
        self.default_repo.clone()
    }
}

impl RequestValidator for ServiceRequest {
    fn is_valid_create(&self) -> RequestValidatorResult {
        match (
            self.name.is_some(),
            self.application.as_ref().is_some(),
            self.default_repo.is_some()
        ) {
            (false, _, _) => Err(RequestValidatorError::MissingAttribute("name".to_string())),
            (_, false, _) => Err(RequestValidatorError::MissingAttribute("application".to_string())),
            (_, _, false) => Err(RequestValidatorError::MissingAttribute("default_repo".to_string())),
            (true, true, true) => self.application.as_ref().unwrap().is_valid_get()
        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        match (
            self.name.is_some(),
            self.application.as_ref().is_some()
        ) {
            (false, _) => Err(RequestValidatorError::MissingAttribute("name".to_string())),
            (_, false) => Err(RequestValidatorError::MissingAttribute("application".to_string())),
            (true, true) => self.application.as_ref().unwrap().is_valid_get()
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.name.is_some(),
            self.application.as_ref().is_some(),
            self.default_repo.is_some()
        ) {
            (false, false, false) => Err(RequestValidatorError::AtLeastOneRequired),
            (_, true, _) => self.application.as_ref().unwrap().is_valid_get(),
            (_, _, _) => Ok(()),
        }
    }
}

impl FromStr for ServiceRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: ServiceRequest =
            serde_json::from_str(s).map_err(|e| format!("error parsing service request: {}", e))?;
        Ok(res)
    }
}