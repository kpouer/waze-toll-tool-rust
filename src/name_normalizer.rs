use std::collections::HashMap;
use crate::io_tools::read_lines;


pub(crate) struct NameNormalizer {
    map: HashMap<String, String>
}

impl NameNormalizer {
    pub(crate) fn new() -> NameNormalizer {
        const ALIAS_FILENAME: &'static str = "prices/alias.csv";
        let mut map: HashMap<String, String> = HashMap::new();
        if let Ok(lines) = read_lines(ALIAS_FILENAME) {
            for line in lines {
                let line = line.unwrap();
                let tokens = line.split(",");
                let tokens = tokens.map(|token| token.to_string()).collect::<Vec<String>>();
                if tokens.len() != 2 {
                    println!("Invalid alias line {}", line);
                    continue;
                }
                let first_token = tokens.get(0).unwrap().to_string();
                let second_token = tokens.get(1).unwrap().to_string();
                map.insert(first_token, second_token);
            }
        } else {
            println!("Unable to proceed : cannot read alias file {}", ALIAS_FILENAME);
            panic!("Unable to proceed : cannot read alias file {}", ALIAS_FILENAME);
        }
        NameNormalizer {
            map
        }
    }

    pub(crate) fn normalize(&self, name: &str) -> String {
        let normalized = name.to_uppercase();
        let normalized = normalized.replace(" - ", " ");
        let normalized = normalized.replace(" / ", " ");
        let normalized = normalized.replace('-', " ");
        let normalized = normalized.replace('/', " ");
        let normalized = normalized.replace('\'', " ");
        if let Some(normalized_name) = self.map.get(&normalized) {
            normalized_name.to_string()
        } else {
            normalized
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_normalize() {
        let name_normalizer = super::NameNormalizer::new();
        assert_eq!("CHATEAU RENAULT", name_normalizer.normalize("CHATEAU-RENAULT"));
    }
}
