#![allow(non_snake_case)]

use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use serde::de::Error;
use serde_json::{from_str, Result};

use crate::json::ToJson;
use crate::toll_file::toll_file::TollFile;

pub(crate) mod toll_file;
mod segment;
pub(crate) mod section;
pub(crate) mod matrix;
pub(crate) mod toll;

pub(crate) fn load_toll_file(toll_file_name: &String) -> Result<TollFile> {
    if let Ok(toll_file) = read_to_string(toll_file_name) {
        let toll_file: TollFile = from_str(&toll_file)?;
        Ok(toll_file)
    } else {
        Err(serde_json::Error::custom(format!("Failed to load toll file {}", toll_file_name)))
    }
}

