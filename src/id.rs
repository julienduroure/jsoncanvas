use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
#[error("ID is empty")]
pub struct EmptyId;

macro_rules! id_type {
    ($($name: ident),+) => { $(
        #[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
        #[repr(transparent)]
        #[serde(transparent)]
        pub struct $name(pub(self) String);

        impl $name {
            pub fn into_inner(self) -> String {
                self.0
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = EmptyId;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if value.is_empty() {
                    Err(EmptyId)
                } else {
                    Ok(Self(value.to_string()))
                }
            }
        }

        impl TryFrom<String> for $name {
            type Error = EmptyId;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.is_empty() {
                    Err(EmptyId)
                } else {
                    Ok(Self(value))
                }
            }
        }
    )+};
}

id_type!(NodeId, EdgeId);
