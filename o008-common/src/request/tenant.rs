use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::RequestValidator;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tenant {
    pub name: Option<String>,
    pub coexisting: Option<bool>
}

impl Tenant {
    pub fn name(&self) -> &str {
        self.name.as_ref().unwrap().as_str()
    }
    
    pub fn coexisting(&self) -> bool {
        *self.coexisting.as_ref().unwrap()
    }
}

impl RequestValidator for Tenant {
    fn is_valid_create(&self) -> bool {
        self.name.is_some() && self.coexisting.is_some()
    }

    fn is_valid_get(&self) -> bool {
        self.name.is_some()
    }
}

impl FromStr for Tenant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Tenant =
            serde_json::from_str(s).map_err(|e| format!("error parsing tenant request: {}", e))?;
        Ok(res)
    }
}
