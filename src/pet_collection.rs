use crate::{common::Pet, errors::PetError};

pub struct PetCollection {
    pets: Vec<Box<dyn Pet>>,
}

impl PetCollection {
    pub fn new() -> Self {
        Self { pets: Vec::new() }
    }

    pub fn add(&mut self, new_pet: Box<dyn Pet>) -> Result<(), PetError> {
        if self.contains(new_pet.name()) {
            return Err(PetError::NameAlreadyExists(new_pet.name().into()));
        }

        self.pets.push(new_pet);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<&dyn Pet, PetError> {
        let matched_pet = self
            .pets
            .iter()
            .find(|pet| pet.name() == name)
            .ok_or(PetError::PetNotFound(name.into()))?;

        Ok(matched_pet.as_ref())
    }

    pub fn get_mut(&mut self, name: &str) -> Result<&mut dyn Pet, PetError> {
        let matched_pet = self
            .pets
            .iter_mut()
            .find(|pet| pet.name() == name)
            .ok_or(PetError::PetNotFound(name.into()))?;

        Ok(matched_pet.as_mut())
    }

    pub fn remove(&mut self, name: &str) -> Result<Box<dyn Pet>, PetError> {
        let index = self
            .pets
            .iter()
            .position(|pet| pet.name() == name)
            .ok_or(PetError::PetNotFound(name.into()))?;

        Ok(self.pets.remove(index))
    }

    pub fn clear(&mut self) {
        self.pets.clear();
    }

    pub fn contains(&self, name: &str) -> bool {
        self.pets.iter().any(|pet| pet.name() == name)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Pet>> {
        self.pets.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Box<dyn Pet>> {
        self.pets.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.pets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pets.is_empty()
    }
}
