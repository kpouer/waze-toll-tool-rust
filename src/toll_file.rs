#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub(crate) struct TollFile {
    pub(crate) tolls: Vec<Toll>
}

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

#[derive(Serialize, Deserialize)]
pub(crate) struct Matrix {
    pub(crate) friendly_name: String,
    pub(crate) matrix_prices: Vec<Vec<f64>>,
    pub(crate) permit_id: String,
    pub(crate) limit_to_vehicles: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Section {
    pub(crate) section_id: String,
    road_local_name: String,
    section_local_name: String,
    location: [f64; 2],
    pub(crate) segments: Vec<Segment>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Segment {
    permalink: String,
    id: u64,
    forwards: bool,
    fromNode: u64,
    toNode: u64,
}

pub(crate) fn load_toll_file(toll_file_name: &String) -> Result<TollFile> {
    let toll_file = std::fs::read_to_string(toll_file_name).unwrap();
    let toll_file: TollFile = serde_json::from_str(&toll_file)?;
    Ok(toll_file)
}