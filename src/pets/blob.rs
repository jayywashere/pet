use crate::{
    common::{Consumable, Pet, PetKind, State},
    data::PetData,
    pets::animal::AnimalKind,
};

pub struct Blob {
    name: String,
    state: State,

    hp: u32,
    hunger: u32,
    happiness: u32,
    gel: u32,
}

impl Blob {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: State::Happy,

            hp: 100,
            hunger: 0,
            happiness: 100,
            gel: 100,
        }
    }

    pub fn restore(
        name: String,
        state: State,
        hp: u32,
        hunger: u32,
        happiness: u32,
        gel: u32,
    ) -> Self {
        Self {
            name,
            state,
            hp,
            hunger,
            happiness,
            gel,
        }
    }

    fn check_gel(&mut self) {
        if self.gel == 0 {
            self.handle_gel_depletion();
        }
    }

    fn handle_gel_depletion(&mut self) {
        self.state = State::Sad;
    }
}

impl Pet for Blob {
    fn pet_kind(&self) -> PetKind {
        PetKind::Blob
    }

    fn animal_kind(&self) -> Option<AnimalKind> {
        None
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn state(&self) -> State {
        self.state
    }

    fn model(&self) -> String {
        match self.state {
            State::Sad => {
                r#"
       .-""""-.
     .'  T  T  '.
    /     __     \
    \   .'  '.   /
     '.        .'
       '-.__.-'
       "#
            }
            _ => {
                r#"
       .-""""-.
     .'  ^  ^  '.
    /    \__/    \
    \   '----'   /
     '.        .'
       '-.__.-'
       "#
            }
        }
        .to_string()
    }

    fn feed(&mut self, item: &dyn Consumable) {
        self.hunger = self.hunger.saturating_sub(item.satiate());
        self.gel = (self.gel + 5).min(100);
    }

    fn play(&mut self) {
        self.happiness = (self.happiness + 15).min(100);
        self.gel = self.gel.saturating_sub(10);

        self.check_gel();
    }

    fn sleep(&mut self) {
        self.gel = (self.gel + 20).min(100);
    }

    fn rename(&mut self, new_name: &str) {
        self.name = new_name.into();
    }

    fn get_stats(&self) -> Vec<(&str, u32)> {
        vec![
            ("HP", self.hp),
            ("Hunger", self.hunger),
            ("Happiness", self.happiness),
            ("Gel", self.gel),
        ]
    }

    fn to_data(&self) -> PetData {
        PetData::Blob {
            name: self.name.clone(),
            state: self.state,
            hp: self.hp,
            hunger: self.hunger,
            happiness: self.happiness,
            gel: self.gel,
        }
    }
}
