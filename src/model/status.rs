use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Ord, PartialOrd, PartialEq, Eq)]
pub(crate) enum Status {
    New,
    Later,
    Ignored,
    Finished,
    Treated,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::New => "New".to_string(),
            Status::Later => "Later".to_string(),
            Status::Ignored => "Ignored".to_string(),
            Status::Finished => "Finished".to_string(),
            Status::Treated => "Treated".to_string()
        };
        write!(f, "{}", str)
    }
}