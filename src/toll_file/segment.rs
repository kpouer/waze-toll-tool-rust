use serde::{Deserialize, Serialize};
use crate::json::ToJson;

#[derive(Serialize, Deserialize)]
pub(crate) struct Segment {
    permalink: String,
    id: u64,
    forwards: bool,
    fromNode: u64,
    toNode: u64,
}

impl ToJson for Segment {
    fn to_json(&self) -> Result<String, String> {
        let mut result = String::new();
        result.push_str("    {\n");
        result.push_str(&format!("      \"permalink\": \"{}\",\n", self.permalink));
        result.push_str(&format!("      \"id\": {},\n", self.id));
        result.push_str(&format!("      \"forwards\": {},\n", self.forwards));
        result.push_str(&format!("      \"fromNode\": {},\n", self.fromNode));
        result.push_str(&format!("      \"toNode\": {}\n    }}", self.toNode));
        Ok(result)
    }
}