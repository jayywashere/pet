use serde::{Deserialize, Serialize};

use crate::{
    common::{Consumable, Food},
    inventory::Inventory,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ConsumableData {
    Food { name: String, satiation: u32 },
}

impl ConsumableData {
    pub fn into_consumable(self) -> Box<dyn Consumable> {
        match self {
            Self::Food { name, satiation } => Box::new(Food::new(name, satiation)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryEntry {
    pub item: ConsumableData,
    pub count: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InventoryData {
    pub items: Vec<InventoryEntry>,
}

pub fn save_inventory(
    inventory: &Inventory,
    path: impl AsRef<std::path::Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = InventoryData {
        items: inventory
            .iter()
            .map(|(_, item)| InventoryEntry {
                item: item.item.to_data(),
                count: item.count,
            })
            .collect(),
    };

    let json = serde_json::to_string_pretty(&data)?;
    std::fs::write(path, json)?;

    Ok(())
}

pub fn load_inventory(
    path: impl AsRef<std::path::Path>,
) -> Result<Inventory, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let data: InventoryData = serde_json::from_str(&json)?;

    let mut inventory = Inventory::new();

    for entry in data.items {
        for _ in 0..entry.count {
            inventory.add(entry.item.clone().into_consumable());
        }
    }

    Ok(inventory)
}
