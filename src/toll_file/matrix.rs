use std::fmt::{Display, format};
use std::str::FromStr;
use chrono::format;
use serde::{Deserialize, Serialize};
use crate::json::ToJson;

#[derive(Serialize, Deserialize)]
pub(crate) struct Matrix {
    pub(crate) friendly_name: String,
    pub(crate) matrix_prices: Vec<Vec<f64>>,
    pub(crate) permit_id: String,
    pub(crate) limit_to_vehicles: Vec<String>
}

impl ToJson for Matrix {
    fn to_json(&self) -> Result<String, String> {
        let mut result = String::new();
        result.push_str("{\n");
        result.push_str(&format!("          \"friendly_name\": \"{}\",\n", self.friendly_name));
        result.push_str("          \"matrix_prices\": [\n");
        for (index, prices) in self.matrix_prices.iter().enumerate() {
            if index > 0 {
                result.push_str(",")
            }
            result.push_str("            [\n");
            for (index, price) in prices.iter().enumerate() {
                if index > 0 {
                    result.push_str(",\n")
                }

                result.push_str(&format!("              {}", format_f64(*price)));
                // result.push_str(&format!("              {}", Currency::from_f64(*price)));
            }
            result.push_str("]\n");
        }
        result.push_str("      ],\n");
        result.push_str(&format!("      \"permit_id\": \"{}\",\n", self.permit_id));
        result.push_str("      \"limit_to_vehicles\": [\n");
        for vehicle in &self.limit_to_vehicles {
            result.push_str(&format!("        \"{}\",\n", vehicle));
        }
        result.push_str("      ]\n    }");
        Ok(result)
    }
}

fn format_f64(value: f64) -> String {
    if value == value.trunc() {
        return format!("{}", value as u8);
    }
    let v = value * 10.0;
    if v == v.trunc() {
        return format!("{:.1}", value);
    }
    let result = format!("{}", value);
    result
}

struct Currency {
    value: i16,
    cents: u8,
}

impl Currency {
    fn new(value: i16, cents: u8) -> Currency {
        Currency {
            value,
            cents
        }
    }

    fn from_f64(src: f64) -> Currency {
        let value = src as i16;
        let cents: i8 = ((src - src.trunc()) * 100.0) as i8;
        Currency {
            value,
            cents: cents as u8
        }
    }

    fn from_str(value: &str) -> Currency {
        let mut parts = value.split('.');
        let value = parts.next().unwrap().parse().unwrap();
        if let Some(cents) = parts.next() {
            return Currency {
                value,
                cents: cents.parse().unwrap()
            }
        }
        Currency {
            value,
            cents: 0
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cents == 0 {
            return write!(f, "{}", format!("{}", self.value))
        }
        if self.cents < 10 {
            return write!(f, "{}", format!("{}.0{}", self.value, self.cents))
        }
        if self.cents % 10 == 0 {
            return write!(f, "{}", format!("{}.{}", self.value, self.cents / 10))
        }
        return write!(f, "{}", format!("{}.{}", self.value, self.cents))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_from_f64_handles_integer_values() {
        let currency = Currency::from_f64(123.0);
        assert_eq!(currency.value, 123);
        assert_eq!(currency.cents, 0);
    }

    #[test]
    fn currency_from_f64_handles_fractional_values() {
        let currency = Currency::from_f64(123.45);
        assert_eq!(currency.value, 123);
        assert_eq!(currency.cents, 45);
    }

    #[test]
    fn currency_from_str_handles_integer_values() {
        let currency = Currency::from_str("123");
        assert_eq!(currency.value, 123);
        assert_eq!(currency.cents, 0);
    }

    #[test]
    fn currency_from_str_handles_fractional_values() {
        let currency = Currency::from_str("123.45");
        assert_eq!(currency.value, 123);
        assert_eq!(currency.cents, 45);
    }

    #[test]
    fn currency_display_handles_integer_values() {
        let currency = Currency::new(123, 0);
        assert_eq!(currency.to_string(), "123");
    }

    #[test]
    fn currency_display_handles_fractional_values() {
        let currency = Currency::new(123, 45);
        assert_eq!(currency.to_string(), "123.45");
    }
}