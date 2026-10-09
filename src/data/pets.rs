use serde::{Deserialize, Serialize};

use crate::{
    common::{Pet, State},
    pet_collection::PetCollection,
    pets::{Animal, Blob, Robot, animal::AnimalKind},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PetData {
    Animal {
        name: String,
        animal_kind: AnimalKind,
        state: State,

        hp: u32,
        hunger: u32,
        happiness: u32,
        energy: u32,
    },

    Blob {
        name: String,
        state: State,

        hp: u32,
        hunger: u32,
        happiness: u32,
        gel: u32,
    },

    Robot {
        name: String,
        state: State,

        hp: u32,
        hunger: u32,
        happiness: u32,
        heat: u32,
    },
}

impl PetData {
    pub fn into_pet(self) -> Box<dyn Pet> {
        match self {
            Self::Animal {
                name,
                animal_kind,
                state,
                hp,
                hunger,
                happiness,
                energy,
            } => Box::new(Animal::restore(
                name,
                animal_kind,
                state,
                hp,
                hunger,
                happiness,
                energy,
            )),

            Self::Blob {
                name,
                state,
                hp,
                hunger,
                happiness,
                gel,
            } => Box::new(Blob::restore(name, state, hp, hunger, happiness, gel)),

            Self::Robot {
                name,
                state,
                hp,
                hunger,
                happiness,
                heat,
            } => Box::new(Robot::restore(name, state, hp, hunger, happiness, heat)),
        }
    }
}

pub fn save_pets(
    collection: &PetCollection,
    path: impl AsRef<std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let pets: Vec<PetData> = collection.iter().map(|pet| pet.to_data()).collect();

    let json = serde_json::to_string_pretty(&pets)?;
    std::fs::write(path, json)?;

    Ok(())
}

pub fn load_pets(
    path: impl AsRef<std::path::Path>,
) -> Result<PetCollection, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let pets: Vec<PetData> = serde_json::from_str(&json)?;

    let mut collection = PetCollection::new();

    for pet in pets {
        collection.add(pet.into_pet())?;
    }

    Ok(collection)
}
