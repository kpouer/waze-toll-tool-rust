use std::collections::HashMap;
use std::{fmt, fs};
use std::fmt::Formatter;
use chrono::{Datelike, Utc};
use crate::category::Category;
use crate::name_normalizer::NameNormalizer;
use crate::price::Price;
use crate::price_grid::{PriceKey, PriceLoader};
use crate::toll_file::{load_toll_file, Matrix, Section, Toll, TollFile};

struct Audit {
    obsolete: u16,
    found: u16,
    not_found: u16
}
pub(crate) struct PriceService {
    prices: HashMap<PriceKey, Price>,
    name_normalizer: NameNormalizer
}

impl PriceService {
    pub(crate) fn new() -> PriceService {
        let name_normalizer = NameNormalizer::new();
        let mut price_loader = PriceLoader {
            name_normalizer: &name_normalizer,
            prices: HashMap::new()
        };
        let audit = price_loader.load_prices();
        println!("Price loader audit : {}", audit);
        let prices = price_loader.prices;
        PriceService {
            prices,
            name_normalizer
        }
    }

    pub(crate) fn get_prices(&self, entry_name: &String) {
        println!("Getting prices for {}", entry_name);
        let mut found_prices = false;
        for (key, value) in &self.prices {
            let key_entry_name = &key.entry;
            if key_entry_name.contains(entry_name) {
                found_prices = true;
                let destination = &key.exit;
                println!("{} {}", destination, value.price);
            }
        }
        if !found_prices {
            println!("No prices found for {}", entry_name);
        }
    }

    pub(crate) fn build_matrix(&self, toll_file_name: &String) {
        println!("Building matrix for {}", toll_file_name);
        let toll_file = load_toll_file(toll_file_name);
        if toll_file.is_ok() {
            let mut toll_file = toll_file.unwrap();
            println!("Loaded toll file {} containing {} toll", toll_file_name, toll_file.tolls.len());
            for toll in &mut toll_file.tolls {
                self.update_toll_matrix(toll)
            }
            write_toll_file(&toll_file, "out.json");
        } else {
            println!("Failed to load toll file {}", toll_file_name);
        }
    }

    fn update_toll_matrix(&self, toll: &mut Toll) {
        println!("Updating toll matrix for {}", toll.toll_id);
        if toll.rules.len() != 1 && toll.rules.get(0).unwrap() != "entry_exit_price" {
            println!("Skipping toll {} because it has {} rules", toll.toll_id, toll.rules.len());
            return;
        }
        let car_matrix = self.build_matrix_category(&toll.sections, Category::Car);
        let motorcycle_matrix = self.build_matrix_category(&toll.sections, Category::Motorcycle);
        toll.entry_exit_matrix = vec![car_matrix.0, motorcycle_matrix.0];
        println!("Car        : Found {} prices, {} obsolete, {} not found",
                 car_matrix.1.found + car_matrix.1.obsolete,
                 car_matrix.1.obsolete,
                 car_matrix.1.not_found);
        println!("Motocycles : Found {} prices, {} obsolete, {} not found",
                 motorcycle_matrix.1.found + motorcycle_matrix.1.obsolete,
                 motorcycle_matrix.1.obsolete,
                 motorcycle_matrix.1.not_found);
    }

    fn build_matrix_category(&self, sections: &Vec<Section>, category: Category) -> (Matrix, Audit) {
        let year = Utc::now().year() as u16;
        let limit_to_vehicles = match category {
            Category::Car => vec!["PRIVATE".to_string(), "TAXI".to_string(), "EV".to_string()],
            Category::Motorcycle => vec!["MOTORCYCLE".to_string()],
        };

        let mut matrix_prices: Vec<Vec<f64>> = Vec::new();
        let mut missing_prices: u16 = 0;
        let mut found_prices: u16 = 0;
        let mut obsolete_prices: u16 = 0;
        for entry_section in sections {
            let mut row = Vec::new();
            let entry_id = self.name_normalizer.normalize(&entry_section.section_id);
            for exit_section in sections {
                let exit_id = self.name_normalizer.normalize(&exit_section.section_id);
                if entry_id == exit_id {
                    row.push(0.0);
                } else {
                    let key = PriceKey {
                        entry: entry_id.to_string(),
                        exit: exit_id.to_string(),
                        category
                    };
                    let price = self.prices.get(&key);
                    if price.is_none() {
                        println!("Unknown price for {}", key);
                        row.push(0.0);
                        missing_prices += 1;
                    } else {
                        let price = price.unwrap();
                        row.push(price.price as f64 / 100 as f64);
                        if year > price.year {
                            println!("Price is obsolete (from {}) for {}, please update file {}", price.year, key, price.file);
                            obsolete_prices += 1;
                        }
                        found_prices += 1;
                    }
                }
            }
            matrix_prices.push(row);
        }
        let audit = Audit {
            obsolete: obsolete_prices,
            found: found_prices,
            not_found: missing_prices
        };
        let matrix = Matrix {
            friendly_name: category.to_string(),
            matrix_prices,
            permit_id: "".to_string(),
            limit_to_vehicles
        };
        (matrix, audit)
    }
}

impl fmt::Display for PriceService {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PriceService : nb-prices={}", self.prices.len())
    }
}

fn write_toll_file(toll: &TollFile, file_name: &str) {
    let json = serde_json::to_string_pretty(&toll).unwrap();
    fs::write(file_name, json).expect("Unable to write file");
    println!("Wrote {}", file_name);
}