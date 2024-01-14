use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::request::{RequestValidator, RequestValidatorError, RequestValidatorResult};
use utoipa::ToSchema;
use crate::TypeInfo;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct BuilderRequest {
    pub name: Option<String>,
    pub active: Option<bool>,
    pub build_command: Option<String>,
}

impl BuilderRequest {
    pub fn name(&self) -> &str {
        self.name.as_ref().unwrap().as_str()
    }

    pub fn active(&self) -> bool {
        *self.active.as_ref().unwrap()
    }

    pub fn build_command(&self) -> &str {
        self.build_command.as_ref().unwrap().as_str()
    }
}

impl RequestValidator for BuilderRequest {
    fn is_valid_create(&self) -> RequestValidatorResult {
        match (
            self.name.as_ref(),
            self.active,
            self.build_command.as_ref()
        ) {
            (Some(_), Some(_), Some(_)) => Ok(()),
            (_, _, _) => Err(RequestValidatorError::MissingAttribute(format!("{} all attributes are mandatory", self.type_of()))),
        }
    }

    fn is_valid_get(&self) -> RequestValidatorResult {
        if self.name.is_some() {
            Ok(())
        } else {
            Err(RequestValidatorError::MissingAttribute(format!("{} name attribute is mandatory", self.type_of())))
        }
    }

    fn is_valid_update(&self) -> RequestValidatorResult {
        match (
            self.name.as_ref(),
            self.active,
            self.build_command.as_ref()
        ) {
            (None, None, None) => Err(RequestValidatorError::MissingAttribute(format!("{} at least one attribute is mandatory", self.type_of()))),
            (_, _, _) => Ok(())
        }
    }
}

const BUILDER_REQUEST_TYPE_INFO: &str = "BuilderRequest";

impl TypeInfo for BuilderRequest {
    fn type_name() -> &'static str {
        BUILDER_REQUEST_TYPE_INFO
    }

    fn type_of(&self) -> &'static str {
        BUILDER_REQUEST_TYPE_INFO
    }
}

impl FromStr for BuilderRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: BuilderRequest =
            serde_json::from_str(s).map_err(|e| format!("error parsing builder request: {}", e))?;
        Ok(res)
    }
}
