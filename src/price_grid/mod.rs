mod price_load_audit;
mod flat_loader;
mod triangle_loader;
mod matrix_loader;
pub(crate) mod currency;
pub(crate) mod toll_file;

use std::collections::HashMap;
use std::{fmt};
use std::fmt::Formatter;
use crate::{DEFAULT_YEAR};
use crate::category::Category;
use crate::name_normalizer::NameNormalizer;
use crate::price::Price;
use crate::price_grid::currency::Currency;
use crate::price_grid::price_load_audit::{PriceLoadAudit};

#[derive(Eq, PartialEq, Hash)]
pub(crate) struct PriceKey {
    pub(crate) entry: String,
    pub(crate) exit: String,
    pub(crate) category: Category
}

struct FlatFileName {
    year: u16,
    entry_index: usize,
    exit_index: usize,
    car_index: usize,
    motorcycle_index: usize,
    file: String
}

pub(crate) struct PriceLoader<'a> {
    pub(crate) name_normalizer: &'a NameNormalizer,
    pub(crate) prices: HashMap<PriceKey, Price>
}

impl<'a> PriceLoader<'a> {
    pub(crate) fn load_prices(&mut self) -> PriceLoadAudit {
         let mut audit = PriceLoadAudit {
            loaded_cars: 0,
            loaded_motorcycles: 0,
            error: Vec::new()
        };
        let new_audit = self.load_flat();
        audit.merge(&new_audit);
        let new_audit = self.load_matrix(Category::Car);
        audit.merge(&new_audit);
        let new_audit = self.load_matrix(Category::Motorcycle);
        audit.merge(&new_audit);
        let new_audit = self.load_triangles(Category::Car);
        audit.merge(&new_audit);
        let new_audit = self.load_triangles(Category::Motorcycle);
        audit.merge(&new_audit);
        audit
    }

    fn insert_price(&mut self, audit: &mut PriceLoadAudit, file: &str, entry: &String, exit: &String, category: Category, price: Currency, year: u16) {
        let key = PriceKey {
            entry: entry.to_string(),
            exit: exit.to_string(),
            category
        };
        let existing_prices = self.prices.get(&key);
        if existing_prices.is_some() {
            let existing_price = existing_prices.unwrap();
            if existing_price.year > year {
                // the existing price is more recent, skip
                return;
            }
        }
        let price = Price {
            price,
            year,
            file: file.to_string()
        };

        match category {
            Category::Car => audit.loaded_cars += 1,
            Category::Motorcycle => audit.loaded_motorcycles += 1
        }
        self.prices.insert(key, price);
    }
}

impl FlatFileName {
    fn new(file_name: &str) -> Result<FlatFileName, String> {
        let end = file_name.find("-").unwrap();
        // suffix : 1,2,4,8
        let suffix = &file_name[end + 1..].strip_suffix(".tsv").unwrap();
        let tokens = suffix
            .split(",")
            .map(|s| s.parse::<u8>().unwrap() - 1)
            .map(|i| i as usize)
            .collect::<Vec<usize>>();


        let flat_file_name = FlatFileName {
            year: get_year(&file_name),
            entry_index: tokens[0],
            exit_index: tokens[1],
            car_index: tokens[2],
            motorcycle_index: tokens[3],
            file: file_name.to_string()
        };
        Ok(flat_file_name)
    }
}

impl fmt::Display for PriceKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}->{}", self.category, self.entry, self.exit)
    }
}

fn get_year(file_name: &str) -> u16 {
    let year = file_name[0..4].parse::<u16>().unwrap_or_else(|_| DEFAULT_YEAR);
    year
}
