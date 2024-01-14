use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{RepoReferenceKind, RequestValidator, TypeInfo};
use crate::request::{RequestValidatorError, RequestValidatorResult};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct RepoReferenceRequest {
    repo: Option<String>,
    kind: Option<RepoReferenceKind>,
    reference: Option<String>,
}

impl RepoReferenceRequest {
    pub fn new(repo: Option<String>, kind: Option<RepoReferenceKind>, reference: Option<String>) -> Self {
        Self {
            repo,
            kind,
            reference,
        }
    }

    pub fn build_get_request(repo: &str, kind: RepoReferenceKind, reference: &str) -> Self {
        Self {
            repo: Some(String::from(repo)),
            kind: Some(kind),
            reference: Some(String::from(reference)),
        }
    }

    pub fn repo(&self) -> Option<&String> {
        self.repo.as_ref()
    }

    pub fn kind(&self) -> Option<&RepoReferenceKind> {
        self.kind.as_ref()
    }

    pub fn reference(&self) -> Option<&String> {
        self.reference.as_ref()
    }
}

impl RequestValidator for RepoReferenceRequest {
    fn is_valid_create(&self) -> RequestValidatorResult {
        match (
            self.repo.as_ref(),
            self.kind.as_ref(),
            self.reference.as_ref()
        ) {
            (Some(_), Some(_), Some(_)) => Ok(()),
            (_, _, _) => Err(RequestValidatorError::MissingAttribute(format!("{} all attributes are mandatory", self.type_of())))
        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        match (
            self.repo.as_ref(),
            self.kind.as_ref(),
            self.reference.as_ref()
        ) {
            (Some(_), Some(_), Some(_)) => Ok(()),
            (_, _, _) => Err(RequestValidatorError::MissingAttribute(format!("{} repo, kind and reference are mandatory", self.type_of())))
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.repo.as_ref(),
            self.kind.as_ref(),
            self.reference.as_ref()
        ) {
            (None, None, None) => Err(RequestValidatorError::MissingAttribute(format!("{} at least one attribute is mandatory", self.type_of()))),
            (_, _, _) => Ok(())
        }
    }
}

const REPO_REFERENCE_REQUEST_TYPE_INFO: &str = "RepoReferenceRequest";

impl TypeInfo for RepoReferenceRequest {
    fn type_name() -> &'static str {
        REPO_REFERENCE_REQUEST_TYPE_INFO
    }

    fn type_of(&self) -> &'static str {
        REPO_REFERENCE_REQUEST_TYPE_INFO
    }
}
