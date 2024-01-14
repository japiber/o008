use std::str::FromStr;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{BuilderRequest, RepoReferenceRequest, RequestValidator, ServiceRequest, TypeInfo};
use crate::request::{RequestValidatorError, RequestValidatorResult};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct ServiceVersionRequest {
    version: Option<String>,
    service: Option<ServiceRequest>,
    repo_ref: Option<RepoReferenceRequest>,
    builder: Option<BuilderRequest>
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct ServiceVersionCreateRequest {
    pub repo_ref: RepoReferenceRequest,
    pub builder: BuilderRequest
}

impl ServiceVersionRequest {
    pub fn new(version: Option<String>, service: Option<ServiceRequest>, repo_ref: Option<RepoReferenceRequest>, builder: Option<BuilderRequest>) -> Self {
        Self {
            version,
            service,
            repo_ref,
            builder
        }
    }

    pub fn build_get_request(version: String, service: String, application: String, tenant: String) -> Self {
        Self {
            version: Some(String::from(version)),
            service: Some(ServiceRequest::build_get_request(service, application, tenant)),
            repo_ref: None,
            builder: None,
        }
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn service(&self) -> Option<&ServiceRequest> {
        self.service.as_ref()
    }

    pub fn repo_ref(&self) -> Option<&RepoReferenceRequest> {
        self.repo_ref.as_ref()
    }

    pub fn builder(&self) -> Option<&BuilderRequest> {
        self.builder.as_ref()
    }

    pub fn set_repo_ref(&mut self, rr: RepoReferenceRequest) {
        self.repo_ref = Some(rr)
    }

    pub fn set_builder(&mut self, builder: BuilderRequest) {
        self.builder = Some(builder)
    }
}

impl RequestValidator for ServiceVersionRequest {
    fn is_valid_create(&self) -> RequestValidatorResult {
        match (
            self.version.as_ref(),
            self.service.as_ref(),
            self.repo_ref.as_ref(),
            self.builder.as_ref()
            ) {
            (Some(_), Some(srv), Some(repo_ref), Some(builder)) => {
                match (srv.is_valid_get(), repo_ref.is_valid_get(), builder.is_valid_get()) {
                    (Ok(()), Ok(()), Ok(())) => Ok(()),
                    (Err(e), _, _) => Err(e),
                    (_, Err(e), _) => Err(e),
                    (_, _, Err(e)) => Err(e),
                }
            },
            (_, _, _, _) => Err(RequestValidatorError::MissingAttribute(format!("{} all attributes are mandatory", self.type_of())))
        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        match (
            self.version.as_ref(),
            self.service.as_ref()
        ) {
            (Some(_), Some(srv)) => srv.is_valid_get(),
            (_, _) => Err(RequestValidatorError::MissingAttribute(format!("{} version and service are mandatory", self.type_of())))
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.version.as_ref(),
            self.service.as_ref(),
            self.repo_ref.as_ref(),
            self.builder.as_ref()
        ) {
            (None, None, None, None) => Err(RequestValidatorError::MissingAttribute(format!("{} at least one is mandatory", self.type_of()))),
            (_, Some(srv), _, _) => srv.is_valid_get(),
            (_, _, Some(repo_ref), _) => repo_ref.is_valid_get(),
            (_, _, _, Some(builder)) => builder.is_valid_get(),
            (_, _, _, _) => Ok(())
        }
    }
}

const SERVICE_VERSION_REQUEST_TYPE_NAME: &str = "ServiceVersionRequest";

impl TypeInfo for ServiceVersionRequest {
    fn type_name() -> &'static str {
        SERVICE_VERSION_REQUEST_TYPE_NAME
    }

    fn type_of(&self) -> &'static str {
        SERVICE_VERSION_REQUEST_TYPE_NAME
    }
}

impl FromStr for ServiceVersionRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let svr : ServiceVersionRequest = serde_json::from_str(s).map_err(|e| format!("error parsing service version request: {}", e))?;
        Ok(svr)
    }
}

impl FromStr for ServiceVersionCreateRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let svr : ServiceVersionCreateRequest = serde_json::from_str(s).map_err(|e| format!("error parsing service version create request: {}", e))?;
        Ok(svr)
    }
}
