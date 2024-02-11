use std::fmt;
use std::fmt::Formatter;
use crate::price_grid::currency::Currency;

#[derive(Clone)]
pub(crate) struct Price {
    pub(crate) price: Currency,
    pub(crate) year: u16,
    pub(crate) file: String
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.year, self.price, self.file)
    }
}