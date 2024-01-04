use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::request::application::Application;
use crate::{ApplicationRequest, RequestValidator, TenantRequest};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Service {
    name: Option<String>,
    application: Option<Application>,
    default_repo: Option<String>,
}

impl Service {
    pub fn new(n: Option<String>, app: Option<Application>, repo: Option<String>) -> Self {
        Self {
            name: n,
            application: app,
            default_repo: repo,
        }
    }

    pub fn get_request(name: String, application: String, tenant: String) -> Self {
        Self::new(
            Some(name),
                Some(ApplicationRequest::new(
                        Some(application),
                        Some(TenantRequest::new(Some(tenant), None)),
                        None,
                        None
                )
            ),
            None,
        )
    }

    pub fn name(&self) -> &str {
        self.name.as_ref().unwrap().as_str()
    }

    pub fn application(&self) -> Application {
        self.application.as_ref().unwrap().clone()
    }

    pub fn default_repo(&self) -> &str {
        self.default_repo.as_ref().unwrap().as_str()
    }
}

impl RequestValidator for Service {
    fn is_valid_create(&self) -> bool {
        self.name.is_some() && self.application.is_some() && self.default_repo.is_some()
    }

    fn is_valid_get(&self) -> bool {
        self.name.is_some() && self.application.is_some()
    }
}

impl FromStr for Service {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Service =
            serde_json::from_str(s).map_err(|e| format!("error parsing service request: {}", e))?;
        Ok(res)
    }
}