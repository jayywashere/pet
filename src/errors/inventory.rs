#[derive(Debug)]
pub enum InventoryError {
    NameIsReserved(String),
    ItemNotFound(String),
    EmptyInventory,
    InvalidQuantity(i32),
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameIsReserved(name) => {
                write!(f, "The name '{}' is reserved!", name)
            }

            Self::ItemNotFound(name) => {
                write!(f, "The name '{}' yielded no item matches!", name)
            }

            Self::EmptyInventory => {
                write!(f, "You still have no items!")
            }

            Self::InvalidQuantity(qty) => {
                write!(f, "Invalid quantity '{}' given!", qty)
            }
        }
    }
}

impl std::error::Error for InventoryError {}
