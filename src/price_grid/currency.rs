use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) struct Currency {
    value: i16,
    cents: i8,
}

impl Currency {
    pub fn new(val: f64) -> Currency {
        let value = val.trunc() as i16;
        let cents = (val.fract() * 100.0).round() as i8;
        Currency { value, cents }
    }

    pub(crate) fn zero() -> Currency {
        Currency { value: 0, cents: 0 }
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
        Ok(Currency::new(val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_currency_with_fractional_value() {
        let currency = Currency::new(12.34);
        assert_eq!(currency.value, 12);
        assert_eq!(currency.cents, 34);
    }

    #[test]
    fn new_currency_with_whole_value() {
        let currency = Currency::new(100.0);
        assert_eq!(currency.value, 100);
        assert_eq!(currency.cents, 0);
    }

    #[test]
    fn zero_currency() {
        let currency = Currency::zero();
        assert_eq!(currency.value, 0);
        assert_eq!(currency.cents, 0);
    }

    #[test]
    fn serialize_currency_with_cents() {
        let currency = Currency::new(12.34);
        let serialized = serde_json::to_string(&currency).unwrap();
        assert_eq!(serialized, "12.34");
    }

    #[test]
    fn serialize_currency_without_cents() {
        let currency = Currency::new(100.0);
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