use crate::{common::PetKind, pets::animal::AnimalKind};

pub struct Args {
    pub command: Command,
}

pub enum Command {
    Help,

    New {
        name: String,
        pet_type: PetKind,
        animal_kind: Option<AnimalKind>,
    },

    View {
        name: Option<String>,
    },

    Feed {
        name: String,
        item: String,
    },

    Play {
        name: Option<String>,
    },

    Sleep {
        name: Option<String>,
    },

    Inventory,

    Rename {
        name: String,
        new_name: String,
    },

    Disown {
        name: Option<String>,
    },

    Shop {
        name: String,
        quantity: Option<i32>,
    },
}
