#[derive(Debug)]
// 'Pet' prefix in the symbol 'PetError' resembles the project name rather than pets themselves, by the way.
pub enum PetError {
    NameAlreadyExists(String),
    NameIsReserved(String),
    PetNotFound(String),
    EmptyPetCollection,
}

impl std::fmt::Display for PetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameAlreadyExists(name) => {
                write!(f, "The name '{}' is already being used!", name)
            }

            Self::NameIsReserved(name) => {
                write!(f, "The name '{}' is reserved!", name)
            }

            Self::PetNotFound(name) => {
                write!(f, "The name '{}' yielded no pet matches!", name)
            }

            Self::EmptyPetCollection => {
                write!(f, "You still have no pets!")
            }
        }
    }
}

impl std::error::Error for PetError {}
