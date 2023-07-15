use std::fs;
use std::fs::DirEntry;
use crate::category::Category;
use crate::io_tools::{is_dir, read_lines_tokens};
use crate::price_grid::price_load_audit::{PriceLoadAudit, PriceLoadError};
use crate::price_grid::{get_year, PriceLoader};

impl<'a> PriceLoader<'a> {
    pub(crate) fn load_matrix(&mut self, category: Category) -> PriceLoadAudit {
        println!("Loading matrix {}", category);
        let path = format!("prices/matrix/{}", category.to_string().to_lowercase());
        let mut audit = PriceLoadAudit::new();
        if is_dir(&path) {
            let paths = fs::read_dir(path).unwrap();
            for path in paths {
                let dir_entry = path.unwrap();
                let new_audit = self.load_matrix_file(category, dir_entry);
                audit.merge(&new_audit);
            }
        } else {
            println!("Directory {} not found", path);
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
}