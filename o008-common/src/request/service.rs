use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::{ApplicationRequest, RequestValidator, TypeInfo};
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
        Self {
            name: Some(name),
            application: Some(ApplicationRequest::build_get_request(application, tenant)),
            default_repo: None
        }
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
            self.name.as_ref(),
            self.application.as_ref(),
            self.default_repo.as_ref()) {
            (Some(_), Some(app), Some(_)) => app.is_valid_get(),
            (_, _, _) => Err(RequestValidatorError::MissingAttribute(format!("{} all attributes are mandatory", self.type_of())))
        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        match (self.name.as_ref(), self.application.as_ref()) {
            (Some(_), Some(app)) => app.is_valid_get(),
            (_,  _) => Err(RequestValidatorError::MissingAttribute(format!("{} name and application are mandatory", self.type_of())))
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.name.as_ref(),
            self.application.as_ref(),
            self.default_repo.as_ref()
        ) {
            (None, None, None) => Err(RequestValidatorError::MissingAttribute(format!("{} at least one is mandatory", self.type_of()))),
            (_, Some(app), _) => app.is_valid_get(),
            (_, None, _) => Ok(()),
        }
    }
}

const SERVICE_REQUEST_TYPE_NAME: &str = "ServiceRequest";

impl TypeInfo for ServiceRequest {
    fn type_name() -> &'static str {
        SERVICE_REQUEST_TYPE_NAME
    }

    fn type_of(&self) -> &'static str {
        SERVICE_REQUEST_TYPE_NAME
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