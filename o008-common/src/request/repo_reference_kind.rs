use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema, Clone, Copy, sqlx::Type)]
pub enum RepoReferenceKind {
    Tag,
    Branch,
    Commit
}

impl Display for RepoReferenceKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoReferenceKind::Tag => write!(f, "Tag"),
            RepoReferenceKind::Branch => write!(f, "Branch"),
            RepoReferenceKind::Commit => write!(f, "Commit"),
        }
    }
}

impl FromStr for RepoReferenceKind {
    type Err = ();
    fn from_str(input: &str) -> Result<RepoReferenceKind, Self::Err> {
        match input {
            "Tag"  => Ok(Self::Tag),
            "Branch"  => Ok(Self::Branch),
            "Commit"  => Ok(Self::Commit),
            _      => Err(()),
        }
    }
}
