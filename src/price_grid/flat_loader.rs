use std::fs;
use enum_iterator::all;
use crate::category::Category;
use crate::io_tools::read_lines;
use crate::price::Price;
use crate::price_grid::price_load_audit::{PriceLoadAudit, PriceLoadError};
use crate::price_grid::{FlatFileName, PriceKey, PriceLoader};

impl<'a> PriceLoader<'a> {
    pub(crate) fn load_flat(&mut self) -> PriceLoadAudit {
        println!("Loading flat prices");
        let mut audit = PriceLoadAudit ::new();
        const FLAT_PRICES_FOLDER: &'static str = "prices/flat";
        if let Ok(paths) = fs::read_dir(FLAT_PRICES_FOLDER) {
            for path in paths {
                let path = path.unwrap().path();
                // 2020_APRR-1,2,4,8.tsv
                let file_name = path.clone();
                let file_name = file_name.file_name().unwrap().to_str().unwrap();
                let flat_file_name = FlatFileName::new(file_name).unwrap();
                println!("Loading {} -> year {}", file_name, flat_file_name.year);
                let mut skipped_header = false;
                if let Ok(lines) = read_lines(path) {
                    for line in lines {
                        if !skipped_header {
                            skipped_header = true;
                            continue;
                        }
                        let line = line.unwrap();
                        let tokens = line.split("\t").collect::<Vec<&str>>();
                        let categories = all::<Category>().collect::<Vec<_>>();
                        for category in categories {
                            if let Ok(result) = self.get_flat_price(&tokens, &flat_file_name, category) {
                                let key = result.0;
                                let value = result.1;
                                self.insert_price(&mut audit, file_name, &key.entry, &key.exit, category, value.price, value.year);
                            } else {
                                let error = PriceLoadError {
                                    file_name: file_name.to_string(),
                                    line: line.to_string(),
                                    error: format!("Invalid line {} for {}", line, category)
                                };
                                audit.error.push(error);
                            }
                        }
                    }
                }
            }
        } else {
            println!("Directory {} not found", FLAT_PRICES_FOLDER);
        }
        audit
    }

    fn get_flat_price(&self, tokens: &Vec<&str>, flat_file_name: &FlatFileName, category: Category) -> Result<(PriceKey, Price), String> {
        let entry = self.name_normalizer.normalize(tokens[flat_file_name.entry_index]);
        let exit = self.name_normalizer.normalize(tokens[flat_file_name.exit_index]);
        let key = PriceKey {
            entry,
            exit,
            category
        };
        let price_index = match category {
            Category::Car => {flat_file_name.car_index}
            Category::Motorcycle => {flat_file_name.motorcycle_index}
        };

        if let Ok(price_value) = tokens[price_index].replace(',',".").parse::<f32>() {
            let price_value = (price_value * 100.) as u16;

            let price = Price {
                price: price_value,
                year: flat_file_name.year,
                file: flat_file_name.file.clone()
            };
            Ok((key, price))
        } else {
            return Err(format!("Invalid price value {} for {}", tokens[price_index], key.entry));
        }
    }
}