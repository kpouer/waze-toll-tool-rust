use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub(crate) struct Currency {
    value: i16,
    cents: i8,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cents == 0 {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{}.{}", self.value, self.cents)
        }
    }
}

impl Currency {
    fn new(value: i16, cents: i8) -> Currency {
        Currency { value, cents }
    }

    pub(crate) fn zero() -> Currency {
        Currency { value: 0, cents: 0 }
    }
}

impl FromStr for Currency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        let value = parts[0].parse::<i16>().unwrap();
        if parts.len() == 1 {
            return Ok(Currency { value, cents: 0 });
        }
        let cents = parts[1].parse::<i8>().unwrap();
        Ok(Currency { value, cents })
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if self.cents == 0 {
            serializer.serialize_i16(self.value)
        } else {
            serializer.serialize_f64(self.value as f64 + self.cents as f64 / 100.0)
        }
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let val = f64::deserialize(deserializer)?;
        let value = (val * 100.0).round() as i16;
        let cents = value % 100;
        let value = ((value - cents) as f64) / 100.0;
        Ok(Currency::new(value as i16, cents as i8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_currency() {
        let currency = Currency::zero();
        assert_eq!(currency.value, 0);
        assert_eq!(currency.cents, 0);
    }

    #[test]
    fn serialize_currency_with_cents() {
        let currency = Currency::new(12, 34);
        let serialized = serde_json::to_string(&currency).unwrap();
        assert_eq!(serialized, "12.34");
    }

    #[test]
    fn serialize_currency_with_cents2() {
        let currency = Currency::new(12, 04);
        let serialized = serde_json::to_string(&currency).unwrap();
        assert_eq!(serialized, "12.04");
    }

    #[test]
    fn serialize_currency_with_cents3() {
        let currency = Currency::new(12, 3);
        let serialized = serde_json::to_string(&currency).unwrap();
        assert_eq!(serialized, "12.3");
    }

    #[test]
    fn serialize_currency_without_cents() {
        let currency = Currency::new(100, 0);
        let serialized = serde_json::to_string(&currency).unwrap();
        assert_eq!(serialized, "100");
    }

    #[test]
    fn deserialize_currency_with_cents() {
        let currency: Currency = serde_json::from_str("9.2").unwrap();
        assert_eq!(currency.value, 9);
        assert_eq!(currency.cents, 20);
    }

    #[test]
    fn deserialize_currency_without_cents() {
        let currency: Currency = serde_json::from_str("100").unwrap();
        assert_eq!(currency.value, 100);
        assert_eq!(currency.cents, 0);
    }
}