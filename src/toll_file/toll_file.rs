use serde::{Deserialize, Serialize};
use crate::json::ToJson;
use crate::toll_file::toll::Toll;

#[derive(Serialize, Deserialize)]
pub(crate) struct TollFile {
    pub(crate) tolls: Vec<Toll>
}

impl ToJson for TollFile {
    fn to_json(&self) -> std::result::Result<String, String> {
        let mut result = String::new();
        result.push_str("{\n  \"tolls\": [\n");
        for toll in &self.tolls {
            result.push_str(&toll.to_json()?);
            result.push_str(",\n");
        }
        result.push_str("  ]\n}");
        Ok(result)
    }
}