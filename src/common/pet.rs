use serde::{Deserialize, Serialize};

use crate::{common::Consumable, data::PetData, pets::animal::AnimalKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetKind {
    Animal,
    Blob,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Hungry,
    Sad,
    Happy,
    Tired,
}

pub trait Pet {
    fn pet_kind(&self) -> PetKind;
    fn animal_kind(&self) -> Option<AnimalKind>;

    fn name(&self) -> &str;
    fn state(&self) -> State;
    fn model(&self) -> String;

    fn feed(&mut self, item: &dyn Consumable);
    fn play(&mut self);
    fn sleep(&mut self);

    fn rename(&mut self, new_name: &str);

    fn get_stats(&self) -> Vec<(&str, u32)>; // Gimmick found per Pet variant, such as: 'heat' for Robot, 'gel' for Blob, and 'energy' for Animal

    fn to_data(&self) -> PetData;
}
