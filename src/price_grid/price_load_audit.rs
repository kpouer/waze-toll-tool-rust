use core::fmt;

pub(crate) struct PriceLoadAudit {
    pub(crate) loaded_cars: u32,
    pub(crate) loaded_motorcycles: u32,
    pub(crate) error: Vec<PriceLoadError>
}

impl PriceLoadAudit {
    pub(crate) fn new() -> PriceLoadAudit {
        PriceLoadAudit {
            loaded_cars: 0,
            loaded_motorcycles: 0,
            error: Vec::new()
        }
    }

    pub(crate) fn merge(&mut self, audit: &PriceLoadAudit) {
        self.loaded_cars += audit.loaded_cars;
        self.loaded_motorcycles += audit.loaded_motorcycles;
        for error in &audit.error {
            self.error.push(error.clone());
        }
    }
}

impl fmt::Display for PriceLoadAudit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loaded {} cars and {} motorcycles", self.loaded_cars, self.loaded_motorcycles)?;
        if self.error.len() > 0 {
            write!(f, "\nErrors:")?;
            for error in &self.error {
                write!(f, "\n\t{}:{}: {}", error.file_name, error.line, error.error)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct PriceLoadError {
    pub(crate) file_name: String,
    pub(crate) line: String,
    pub(crate) error: String
}