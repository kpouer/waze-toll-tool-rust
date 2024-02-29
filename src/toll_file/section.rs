use serde::{Deserialize, Serialize};
use crate::json::ToJson;
use crate::toll_file::segment::Segment;

#[derive(Serialize, Deserialize)]
pub(crate) struct Section {
    pub(crate) section_id: String,
    road_local_name: String,
    section_local_name: String,
    location: [f64; 2],
    pub(crate) segments: Vec<Segment>
}

impl ToJson for Section {
    fn to_json(&self) -> Result<String, String> {
        let mut result = String::new();
        result.push_str("    {\n");
        result.push_str(&format!("      \"section_id\": \"{}\",\n", self.section_id));
        result.push_str(&format!("      \"road_local_name\": \"{}\",\n", self.road_local_name));
        result.push_str(&format!("      \"section_local_name\": \"{}\",\n", self.section_local_name));
        result.push_str(&format!("      \"location\": [{}, {}],\n", self.location[0], self.location[1]));
        result.push_str("      \"segments\": [\n");
        for segment in &self.segments {
            result.push_str("        ");
            result.push_str(&segment.to_json()?);
            result.push_str(",\n");
        }
        result.push_str("      ]\n    }");
        Ok(result)
    }
}
