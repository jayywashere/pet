use std::collections::HashMap;

use crate::{
    common::{Consumable, Food, item::Item},
    errors::InventoryError,
};

pub struct Inventory {
    items: HashMap<String, Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: Box<dyn Consumable>) {
        let name = item.name().to_string();

        self.items
            .entry(name)
            .and_modify(|existing_item| existing_item.count += 1)
            .or_insert(Item { item, count: 1 });
    }

    pub fn get(&mut self, name: &str) -> Result<&dyn Consumable, InventoryError> {
        self.items
            .get(name)
            .map(|item| item.item.as_ref())
            .ok_or(InventoryError::ItemNotFound(name.into()))
    }

    pub fn get_mut(&mut self, name: &str) -> Result<&mut dyn Consumable, InventoryError> {
        match self.items.get_mut(name) {
            Some(item) => Ok(&mut *item.item),
            None => Err(InventoryError::ItemNotFound(name.into())),
        }
    }

    pub fn take(&mut self, name: &str) -> Result<Box<dyn Consumable>, InventoryError> {
        let item = self
            .items
            .get_mut(name)
            .ok_or(InventoryError::ItemNotFound(name.into()))?;

        if item.count > 1 {
            item.count -= 1;

            let fresh_food = Food::new(name.into(), item.item.satiate());

            Ok(Box::new(fresh_food))
        } else {
            let removed = self.items.remove(name).unwrap();
            Ok(removed.item)
        }
    }

    pub fn empty(&mut self, name: &str) -> Result<Box<dyn Consumable>, InventoryError> {
        let item = self
            .items
            .remove(name)
            .ok_or(InventoryError::ItemNotFound(name.into()))?;

        Ok(item.item)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.items.contains_key(name)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Item> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
