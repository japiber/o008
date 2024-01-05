use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::request::RequestValidator;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Builder {
    pub name: Option<String>,
    pub active: Option<bool>,
    pub build_command: Option<String>,
}

impl Builder {
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

impl RequestValidator for Builder {
    fn is_valid_create(&self) -> bool {
        self.name.is_some() &&
            self.active.is_some() &&
            self.build_command.is_some()
    }

    fn is_valid_get(&self) -> bool {
        self.name.is_some()
    }
}

impl FromStr for Builder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Builder =
            serde_json::from_str(s).map_err(|e| format!("error parsing builder request: {}", e))?;
        Ok(res)
    }
}
