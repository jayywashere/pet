use directories::BaseDirs;

use crate::{
    cli::{
        args::{Args, Command},
        commands, parser,
    },
    data::{inventory, pets},
    inventory::Inventory,
    pet_collection::PetCollection,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let Args { command } = parser::parse(std::env::args().skip(1))?;

    let data_dir = get_data_dir()?;
    std::fs::create_dir_all(&data_dir)?;

    let pets_path = data_dir.join("pets.json");
    let inventory_path = data_dir.join("inventory.json");

    let mut collection = if pets_path.exists() {
        pets::load_pets(&pets_path)?
    } else {
        let collection = PetCollection::new();
        pets::save_pets(&collection, &pets_path)?;
        collection
    };

    let mut inventory = if inventory_path.exists() {
        inventory::load_inventory(&inventory_path)?
    } else {
        let inventory = Inventory::new();
        inventory::save_inventory(&inventory, &inventory_path)?;
        inventory
    };

    execute(command, &mut collection, &mut inventory)?;

    pets::save_pets(&collection, &pets_path)?;
    inventory::save_inventory(&inventory, &inventory_path)?;

    Ok(())
}

fn get_data_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| std::io::Error::other("Could not determinate the user's data directory!"))?;

    Ok(base_dirs.data_local_dir().join("pet"))
}

fn execute(
    command: Command,
    collection: &mut PetCollection,
    inventory: &mut Inventory,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Help => {
            println!("{}", commands::help());
        }

        Command::New {
            name,
            pet_type,
            animal_kind,
        } => commands::new(name, collection, pet_type, animal_kind)?,

        Command::View { name } => {
            commands::view(name.as_deref(), collection)?;
        }

        Command::Feed { name, item } => {
            commands::feed(&name, &item, collection, inventory)?;
        }

        Command::Play { name } => {
            commands::play(name.as_deref(), collection)?;
        }

        Command::Sleep { name } => {
            commands::sleep(name.as_deref(), collection)?;
        }

        Command::Inventory => {
            commands::inventory(inventory)?;
        }

        Command::Rename { name, new_name } => {
            commands::rename(&name, &new_name, collection)?;
        }

        Command::Disown { name } => {
            commands::disown(name.as_deref(), collection)?;
        }

        Command::Shop { name, quantity } => {
            commands::shop(&name, quantity, inventory)?;
        }
    }

    Ok(())
}
