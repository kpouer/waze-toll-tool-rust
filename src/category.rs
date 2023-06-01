use enum_iterator::{Sequence};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub(crate) enum Category {
    Car,
    Motorcycle
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Car => write!(f, "Car"),
            Category::Motorcycle => write!(f, "Motorcycle")
        }
    }
}