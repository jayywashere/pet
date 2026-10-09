use crate::data::ConsumableData;

pub trait Consumable {
    fn name(&self) -> &str;
    fn satiate(&self) -> u32;

    fn box_clone(&self) -> Box<dyn Consumable>;

    fn to_data(&self) -> ConsumableData;
}

pub struct Item {
    pub item: Box<dyn Consumable>,
    pub count: usize,
}

pub struct Food {
    name: String,
    satiation: u32,
}

impl Food {
    pub fn new(name: String, satiation: u32) -> Self {
        Self { name, satiation }
    }
}

impl Consumable for Food {
    fn name(&self) -> &str {
        &self.name
    }

    fn satiate(&self) -> u32 {
        self.satiation
    }

    fn box_clone(&self) -> Box<dyn Consumable> {
        Box::new(Food {
            name: self.name.clone(),
            satiation: self.satiation,
        })
    }

    fn to_data(&self) -> ConsumableData {
        ConsumableData::Food {
            name: self.name.clone(),
            satiation: self.satiation,
        }
    }
}
