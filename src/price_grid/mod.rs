mod price_load_audit;

use std::collections::HashMap;
use std::{fmt, fs};
use std::fmt::Formatter;
use std::fs::DirEntry;
use enum_iterator::{all};
use crate::{DEFAULT_YEAR};
use crate::category::Category;
use crate::io_tools::{read_lines, read_lines_tokens};
use crate::name_normalizer::NameNormalizer;
use crate::price::Price;
use crate::price_grid::price_load_audit::{PriceLoadAudit, PriceLoadError};

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
        let new_audit = self.load_triangle(Category::Car);
        audit.merge(&new_audit);
        let new_audit = self.load_triangle(Category::Motorcycle);
        audit.merge(&new_audit);
        audit
    }

    fn load_flat(&mut self) -> PriceLoadAudit {
        println!("Loading flat prices");
        let paths = fs::read_dir("prices/flat").unwrap();
        let mut audit = PriceLoadAudit ::new();
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

    fn load_matrix(&mut self, category: Category) -> PriceLoadAudit {
        println!("Loading matrix {}", category);
        let path = format!("prices/onedirmatrix/{}", category.to_string().to_lowercase());
        let mut audit = PriceLoadAudit::new();
        if !fs::metadata(&path).unwrap().is_dir() {
            println!("Directory {} not found", path);
            return audit;
        }
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            let dir_entry = path.unwrap();
            let new_audit = self.load_matrix_file(category, dir_entry);
            audit.merge(&new_audit);
        }
        audit
    }

    fn load_matrix_file(&mut self, category: Category, dir_entry: DirEntry) -> PriceLoadAudit {
        let path = dir_entry.path();
        let file_name = path.clone();
        let file_name = file_name.file_name().unwrap().to_str().unwrap();
        let year = get_year(&file_name);
        println!("Loading matrix {} -> year {}", file_name, year);
        let mut audit = PriceLoadAudit::new();
        if let Ok(tokenized_lines) = read_lines_tokens(path) {
            let header_line_tokens = &tokenized_lines[0];
            for row in 1..tokenized_lines.len() {
                let line_token = &tokenized_lines[row];
                let entry = self.name_normalizer.normalize(&line_token[0]);
                if line_token.len() != header_line_tokens.len() {
                    println!("Invalid line length for {}", entry);
                    let error = PriceLoadError {
                        file_name: file_name.to_string(),
                        line: "".to_string(),
                        error: "Invalid line length".to_string()
                    };
                    audit.error.push(error);
                    continue;
                }
                for column in 1..line_token.len() {
                    let exit = self.name_normalizer.normalize(&header_line_tokens[column]);
                    let price = line_token[column].replace(',', ".");
                    let price = (price.parse::<f32>().unwrap() * 100.) as u16;
                    self.insert_price(&mut audit, file_name, &entry, &exit, category, price, year)
                }
            }
        }
        audit
    }

    fn load_triangle(&mut self, category: Category) -> PriceLoadAudit{
        println!("Loading triangle matrix");
        let path = format!("prices/triangle/{}", category.to_string().to_lowercase());
        let paths = fs::read_dir(path).unwrap();
        let mut audit = PriceLoadAudit::new();
        for path in paths {
            let path = path.unwrap().path();
            let file_name = path.clone();
            let file_name = file_name.file_name().unwrap().to_str().unwrap();
            let year = get_year(&file_name);
            println!("Loading triangle {} -> year {}", file_name, year);
            if let Ok(tokenized_lines) = read_lines_tokens(path) {
                for row in 0..tokenized_lines.len() {
                    let line_token = &tokenized_lines[row];
                    let entry = self.name_normalizer.normalize(&line_token[line_token.len() - 1]);
                    for column in row + 1..line_token.len() {
                        let line_tokens_2 = &tokenized_lines[column];
                        let exit = self.name_normalizer.normalize(&line_tokens_2[line_tokens_2.len() - 1]);
                        if let Ok(value) = line_tokens_2[row].parse::<f32>() {
                            let value = (value * 100.) as u16;
                            self.insert_price(&mut audit, file_name, &entry, &exit, category, value, year);
                            self.insert_price(&mut audit, file_name, &exit, &entry, category, value, year);
                        } else {
                            println!("Invalid price for {} -> {}", entry, exit);
                            let error = PriceLoadError {
                                file_name: file_name.to_string(),
                                line: "".to_string(),
                                error: "Invalid line length".to_string()
                            };
                            audit.error.push(error);
                        }
                    }
                }
            }
        }
        audit
    }

    fn insert_price(&mut self, audit: &mut PriceLoadAudit, file: &str, entry: &String, exit: &String, category: Category, price: u16, year: u16) {
        let key = PriceKey {
            entry: entry.to_string(),
            exit: exit.to_string(),
            category
        };
        let existing_prices = self.prices.get(&key);
        if existing_prices.is_some() {
            let existing_price = existing_prices.unwrap();
            if existing_price.year > year {
                // println!("Existing price is more recent for {} -> {}", entry, exit);
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
