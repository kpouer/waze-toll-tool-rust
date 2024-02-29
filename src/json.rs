pub(crate) trait ToJson {
    fn to_json(&self) -> Result<String, String>;
}