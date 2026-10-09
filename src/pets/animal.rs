use serde::{Deserialize, Serialize};

use crate::{
    common::{Consumable, Pet, PetKind, State},
    data::PetData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimalKind {
    Dog,
    Cat,
}

pub struct Animal {
    name: String,
    kind: AnimalKind,
    state: State,

    hp: u32,
    hunger: u32,
    happiness: u32,
    energy: u32,
}

impl Animal {
    pub fn new(name: String, kind: AnimalKind) -> Self {
        Self {
            name,
            kind,
            state: State::Happy,

            hp: 100,
            hunger: 0,
            happiness: 100,
            energy: 100,
        }
    }

    pub fn restore(
        name: String,
        kind: AnimalKind,
        state: State,
        hp: u32,
        hunger: u32,
        happiness: u32,
        energy: u32,
    ) -> Self {
        Self {
            name,
            kind,
            state,
            hp,
            hunger,
            happiness,
            energy,
        }
    }

    fn check_energy(&mut self) {
        if self.energy == 0 {
            self.handle_energy_depletion();
        }
    }

    fn handle_energy_depletion(&mut self) {
        self.state = State::Tired;
    }
}

impl Pet for Animal {
    fn pet_kind(&self) -> PetKind {
        PetKind::Animal
    }

    fn animal_kind(&self) -> Option<AnimalKind> {
        Some(self.kind)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn state(&self) -> State {
        self.state
    }

    fn model(&self) -> String {
        let art = match (self.kind, self.state) {
            // Dog States
            (AnimalKind::Dog, State::Tired) => {
                r#"
    / \__/\
    ( -.-  \___
     /         O
    /   (_____/
    /_____/   U"#
            }
            (AnimalKind::Dog, _) => {
                r#"
    / \__/\
    ( o.o  \___
     /         O
    /   (_____/
    /_____/   U
    "#
            }

            // Cat States
            (AnimalKind::Cat, State::Tired) => {
                r#"
     /\_/\
    ( -.- )
     > _ <
     "#
            }
            (AnimalKind::Cat, _) => {
                r#"
     /\_/\
    ( o.o )
     > ^ <
     "#
            }
        };

        art.to_string()
    }

    fn feed(&mut self, item: &dyn Consumable) {
        self.hunger = self.hunger.saturating_sub(item.satiate());
    }

    fn play(&mut self) {
        self.energy = self.energy.saturating_sub(20);
        self.happiness =
            (self.happiness + if self.kind == AnimalKind::Dog { 30 } else { 15 }).min(100);

        self.check_energy();
    }

    fn sleep(&mut self) {
        self.energy = (self.energy + if self.kind == AnimalKind::Cat { 80 } else { 60 }).min(100);

        if self.state == State::Tired {
            self.state = State::Happy;
        }
    }

    fn rename(&mut self, new_name: &str) {
        self.name = new_name.into();
    }

    fn get_stats(&self) -> Vec<(&str, u32)> {
        vec![
            ("HP", self.hp),
            ("Hunger", self.hunger),
            ("Happiness", self.happiness),
            ("Energy", self.energy),
        ]
    }

    fn to_data(&self) -> PetData {
        PetData::Animal {
            name: self.name.clone(),
            animal_kind: self.kind,
            state: self.state,
            hp: self.hp,
            hunger: self.hunger,
            happiness: self.happiness,
            energy: self.energy,
        }
    }
}
