use crate::{
    common::{Consumable, Pet, PetKind, State},
    data::PetData,
    pets::animal::AnimalKind,
};

pub struct Robot {
    name: String,
    state: State,

    hp: u32,
    hunger: u32,
    happiness: u32,
    heat: u32,
}

impl Robot {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: State::Happy,

            hp: 100,
            hunger: 0,
            happiness: 100,
            heat: 0,
        }
    }

    pub fn restore(
        name: String,
        state: State,
        hp: u32,
        hunger: u32,
        happiness: u32,
        heat: u32,
    ) -> Self {
        Self {
            name,
            state,
            hp,
            hunger,
            happiness,
            heat,
        }
    }

    fn check_heat(&mut self) {
        if self.heat == 100 {
            self.handle_overheat();
        }
    }

    fn handle_overheat(&mut self) {
        self.hp = self.hp.saturating_sub(20);
        self.heat = 0;
    }
}

impl Pet for Robot {
    fn pet_kind(&self) -> PetKind {
        PetKind::Robot
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
        r#"
    .---------.
    |  ^  ^   |
    |   \/    |
    '---| |---'
       /| |\
      /_| |_\
        "#
        .trim_end()
        .to_string()
    }

    fn feed(&mut self, item: &dyn Consumable) {
        self.hunger = self.hunger.saturating_sub(item.satiate());
        self.heat = (self.heat + 5).min(100);

        self.check_heat();
    }

    fn play(&mut self) {
        self.happiness = (self.happiness + 10).min(100);
        self.heat = (self.heat + 5).min(100);

        self.check_heat();
    }

    fn sleep(&mut self) {
        self.heat = self.heat.saturating_sub(30);
    }

    fn rename(&mut self, new_name: &str) {
        self.name = new_name.into();
    }

    fn get_stats(&self) -> Vec<(&str, u32)> {
        vec![
            ("HP", self.hp),
            ("Hunger", self.hunger),
            ("Happiness", self.happiness),
            ("Heat", self.heat),
        ]
    }

    fn to_data(&self) -> PetData {
        PetData::Robot {
            name: self.name.clone(),
            state: self.state,
            hp: self.hp,
            hunger: self.hunger,
            happiness: self.happiness,
            heat: self.heat,
        }
    }
}
