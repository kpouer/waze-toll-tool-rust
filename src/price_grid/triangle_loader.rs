use std::fs;
use std::path::PathBuf;
use crate::category::Category;
use crate::io_tools::{is_dir, read_lines_tokens};
use crate::price_grid::price_load_audit::{PriceLoadAudit, PriceLoadError};
use crate::price_grid::{get_year, PriceLoader};
use crate::price_grid::currency::Currency;

impl<'a> PriceLoader<'a> {
    pub(crate) fn load_triangles(&mut self, category: Category) -> PriceLoadAudit {
        println!("Loading triangle matrix {}", category);
        let path = format!("prices/triangle/{}", category.to_string().to_lowercase());
        let mut audit = PriceLoadAudit::new();
        if is_dir(&path) {
            let paths = fs::read_dir(path).unwrap();
            for path in paths {
                let path = path.unwrap().path();
                let triangle_audit = self.load_triangle(category, path);
                audit.merge(&triangle_audit);
            }
        } else {
            println!("Directory {} not found", path);
        }
        audit
    }

    fn load_triangle(&mut self, category: Category, path: PathBuf) -> PriceLoadAudit {
        let file_name = path.clone();
        let file_name = file_name.file_name().unwrap().to_str().unwrap();
        let year = get_year(&file_name);
        let mut audit = PriceLoadAudit::new();
        println!("Loading triangle {} -> year {}", file_name, year);
        if let Ok(tokenized_lines) = read_lines_tokens(path) {
            let row_count = tokenized_lines.len();
            for row in 0..row_count {
                let line_token = &tokenized_lines[row];
                let entry = self.name_normalizer.normalize(&line_token[line_token.len() - 1]);
                for column in row + 1..row_count {
                    let line_tokens_2 = &tokenized_lines[column];
                    let exit = self.name_normalizer.normalize(&line_tokens_2[line_tokens_2.len() - 1]);
                    let price_token = &line_tokens_2[row].replace(',', ".");
                    if let Ok(value) = price_token.parse::<Currency>() {
                        self.insert_price(&mut audit, file_name, &entry, &exit, category, value.clone(), year);
                        self.insert_price(&mut audit, file_name, &exit, &entry, category, value, year);
                    } else {
                        println!("Invalid price for {} -> {} : {}", entry, exit, price_token);
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
        audit
    }
}