use crate::{
    common::{Food, Pet, PetKind},
    errors::{InventoryError, PetError},
    inventory::Inventory,
    pet_collection::PetCollection,
    pets::{Animal, Blob, Robot, animal::AnimalKind},
};

use rand::RngExt;

// pet help / pet --help
pub fn help() -> String {
    r#"
    pet - a virtual pet CLI

    Usage:
    pet <COMMAND> [OPTIONS]
    pet help

    COMMANDS:
    new - Create a new pet
    view - View one or all pets
    feed - Feed a pet using an inventory item
    play - Play with one or all pets
    sleep - Let one or all pets sleep
    inventory - View your inventory
    rename - Rename a pet
    disown - Disown one or all pets
    shop - Create items for your inventory

    EXAMPLES:
    pet new --name Mochi --kind animal --animalKind Dog
    pet new --name Choco --kind blob
    |
    pet view
    pet view --name Mochi
    |
    pet feed --petName Choco --itemName apple
    |
    pet play
    pet play --name Mochi
    |
    pet sleep
    pet sleep --name Choco
    |
    pet inventory
    |
    pet rename --name Mochi --newName Dango
    |
    pet disown --name Choco
    |
    pet shop --name apple
    pet shop --name milk --quantity 10
    "#
    .into()
}

// pet new --name <name> --kind <"animal"/"blob"/"robot"> --animalKind (if kind is "animal") <"dog"/"cat">
pub fn new(
    name: String,
    collection: &mut PetCollection,
    kind: PetKind,
    animal_kind: Option<AnimalKind>,
) -> Result<(), PetError> {
    if name == "all" {
        return Err(PetError::NameIsReserved("all".into()));
    }

    match kind {
        PetKind::Animal => {
            let ak = match animal_kind {
                Some(kind) => kind,
                None => unreachable!(
                    "Frontend guarantees that 'animal_kind' is always present for 'PetKind::Animal'"
                ),
            };

            collection.add(Box::new(Animal::new(name, ak)))?;
        }

        PetKind::Blob => {
            collection.add(Box::new(Blob::new(name)))?;
        }

        PetKind::Robot => {
            collection.add(Box::new(Robot::new(name)))?;
        }
    }

    Ok(())
}

// pet view (--name <name>)? (if not supplied, then view all)
pub fn view(name: Option<&str>, collection: &PetCollection) -> Result<(), PetError> {
    match name {
        // View specific
        Some(name) => {
            let pet = collection.get(name)?;
            view_specific(pet);
        }

        // View all
        None => {
            if collection.is_empty() {
                return Err(PetError::EmptyPetCollection);
            }

            for pet in collection.iter() {
                view_specific(pet.as_ref());
            }
        }
    }

    Ok(())
}

// Helper for view()
fn view_specific(pet: &dyn Pet) {
    // ASCII Model
    println!("{}", pet.model());

    // Profile
    println!("\nProfile:");
    println!("Name: {}", pet.name());
    println!("Kind: {:?}", pet.pet_kind());
    if let Some(ak) = pet.animal_kind() {
        println!("Animal Kind: {:?}", ak);
    }
    println!("State: {:?}\n", pet.state());

    // Stats
    println!("Stats:");
    let stats = pet.get_stats();
    for (key, val) in stats {
        println!("{key}: {val}");
    }
}

// pet feed --petName <name> --itemName <name>
pub fn feed(
    pet_name: &str,
    item_name: &str,
    collection: &mut PetCollection,
    inventory: &mut Inventory,
) -> Result<(), Box<dyn std::error::Error>> {
    let pet = collection.get_mut(pet_name)?;
    let item = inventory.take(item_name)?;

    pet.feed(item.as_ref());

    Ok(())
}

// pet play (--name <name>)? (if not supplied, then make all play)
pub fn play(name: Option<&str>, collection: &mut PetCollection) -> Result<(), PetError> {
    match name {
        Some(name) => {
            let pet = collection.get_mut(name)?;
            pet.play();
        }

        None => {
            if collection.is_empty() {
                return Err(PetError::EmptyPetCollection);
            }

            for pet in collection.iter_mut() {
                pet.as_mut().play()
            }
        }
    }

    Ok(())
}

// pet sleep (--name <name>)? (if not supplied, then make all sleep)
pub fn sleep(name: Option<&str>, collection: &mut PetCollection) -> Result<(), PetError> {
    match name {
        Some(name) => {
            let pet = collection.get_mut(name)?;
            pet.sleep();
        }

        None => {
            if collection.is_empty() {
                return Err(PetError::EmptyPetCollection);
            }

            for pet in collection.iter_mut() {
                pet.as_mut().sleep();
            }
        }
    }

    Ok(())
}

// pet viewInventory
pub fn inventory(inventory: &Inventory) -> Result<(), InventoryError> {
    if inventory.is_empty() {
        return Err(InventoryError::EmptyInventory);
    }

    for (name, item) in inventory.iter() {
        println!("{} (x{})", name, item.count);
    }

    Ok(())
}

// pet rename --petName <name> --newName <name>
pub fn rename(
    pet_name: &str,
    new_name: &str,
    collection: &mut PetCollection,
) -> Result<(), PetError> {
    if new_name == "all" {
        return Err(PetError::NameIsReserved("all".into()));
    }

    if collection.contains(new_name) {
        return Err(PetError::NameAlreadyExists(new_name.into()));
    }

    let pet = collection.get_mut(pet_name)?;
    pet.rename(new_name);

    Ok(())
}

// pet disown (--petName <name>)? (if not supplied, disown all)
pub fn disown(name: Option<&str>, collection: &mut PetCollection) -> Result<(), PetError> {
    match name {
        Some(name) => {
            collection.remove(name)?;
        }

        None => {
            if collection.is_empty() {
                return Err(PetError::EmptyPetCollection);
            }

            collection.clear();
        }
    }

    Ok(())
}

// pet shop --name <name> (--quantity <amount>)? (if not supplied, then default to 1)
pub fn shop(
    name: &str,
    qty: Option<i32>,
    inventory: &mut Inventory,
) -> Result<(), Box<dyn std::error::Error>> {
    if name == "all" {
        return Err(PetError::NameIsReserved("all".into()).into());
    }

    let qty = match qty {
        Some(qty) => qty,
        None => 1,
    };

    if qty < 1 {
        return Err(InventoryError::InvalidQuantity(qty).into());
    }

    let qty = qty as u32;

    let mut rng = rand::rng();
    let satiation = rng.random_range(1..=50);

    for _ in 0..qty {
        let food = Food::new(name.into(), satiation);
        inventory.add(Box::new(food));
    }

    Ok(())
}
