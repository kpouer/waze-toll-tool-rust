use serde::{Deserialize, Serialize};
use crate::json::ToJson;
use crate::toll_file::matrix::Matrix;
use crate::toll_file::section::Section;

#[derive(Serialize, Deserialize)]
pub(crate) struct Toll {
    pub(crate) toll_id: String,
    road_local_name: String,
    currency: String,
    currency_code: String,
    polyline: String,
    r#type: String,
    pub(crate) rules: Vec<String>,
    pub(crate) entry_exit_matrix: Vec<Matrix>,
    pub(crate) sections: Vec<Section>
}

impl ToJson for Toll {
    fn to_json(&self) -> Result<String, String> {
        let mut result = String::new();
        result.push_str("    {\n");
        result.push_str(&format!("      \"toll_id\": \"{}\",\n", self.toll_id));
        result.push_str(&format!("      \"road_local_name\": \"{}\",\n", self.road_local_name));
        result.push_str(&format!("      \"currency\": \"{}\",\n", self.currency));
        result.push_str(&format!("      \"currency_code\": \"{}\",\n", self.currency_code));
        result.push_str(&format!("      \"polyline\": \"{}\",\n", self.polyline));
        result.push_str(&format!("      \"type\": \"{}\",\n", self.r#type));
        result.push_str("      \"rules\": [");
        for (index, rule) in self.rules.iter().enumerate() {
            if index > 0 {
                result.push_str(",")
            }
            result.push_str(&format!("\n        \"{}\"", rule));
        }
        result.push_str("\n      ],\n");

        result.push_str("      \"entry_exit_matrix\": [\n");
        for (index, matrix) in self.entry_exit_matrix.iter().enumerate() {
            if index > 0 {
                result.push_str(",")
            }
            result.push_str("        ");
            result.push_str(&matrix.to_json()?);
            result.push_str(",\n");
        }
        result.push_str("      ],\n");
        result.push_str("      \"sections\": [\n");
        for section in &self.sections {
            result.push_str("        ");
            result.push_str(&section.to_json()?);
            result.push_str(",\n");
        }
        result.push_str("      ]\n    }");
        Ok(result)
    }
}