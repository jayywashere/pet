use crate::{
    cli::args::{Args, Command},
    common::PetKind,
    errors::ParserError,
    pets::animal::AnimalKind,
};

pub fn parse(arguments: impl Iterator<Item = String>) -> Result<Args, Box<dyn std::error::Error>> {
    let mut arguments = arguments.peekable();

    let command_name = arguments.next();

    let command: Command = match command_name.as_deref() {
        Some("help") => Command::Help,

        Some("new") => parse_new(&mut arguments)?,

        Some("view") => parse_view(&mut arguments)?,

        Some("feed") => parse_feed(&mut arguments)?,

        Some("play") => parse_play(&mut arguments)?,

        Some("sleep") => parse_sleep(&mut arguments)?,

        Some("inventory") => Command::Inventory,

        Some("rename") => parse_rename(&mut arguments)?,

        Some("disown") => parse_disown(&mut arguments)?,

        Some("shop") => parse_shop(&mut arguments)?,

        Some(cmd) => {
            return Err(ParserError::UnknownCommand(cmd.into()).into());
        }

        None => {
            return Err(ParserError::MissingCommand.into());
        }
    };

    if let Some(extra) = arguments.next() {
        return Err(ParserError::UnknownArgument {
            argument: extra,
            flag: command_name.unwrap_or_else(|| "command".into()),
        }
        .into());
    }

    Ok(Args { command })
}

fn parse_new(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> Result<Command, ParserError> {
    // Initial: (pet) (new) --name <name> --kind <"animal"/"blob"/"robot"> --animalKind (if kind is "animal") <"dog"/"cat">

    let (_, name) = parse_kv(args, "--name")?;
    let (flag, kind) = parse_kv(args, "--kind")?;

    let reason = format!("Expected 'animal', 'blob', or 'robot', but got '{kind}'!");
    let pet_type = match kind.as_str() {
        "animal" => PetKind::Animal,
        "blob" => PetKind::Blob,
        "robot" => PetKind::Robot,
        _ => {
            return Err(ParserError::InvalidArgument {
                argument: kind.clone(),
                flag,
                reason,
            });
        }
    };

    let animal_kind = if kind == "animal" {
        let (flag, animal) = parse_kv(args, "--animalKind")?;
        let reason = format!("Expected 'dog' or 'cat', but got '{animal}'!");

        Some(match animal.as_str() {
            "dog" => AnimalKind::Dog,
            "cat" => AnimalKind::Cat,
            _ => {
                return Err(ParserError::InvalidArgument {
                    argument: animal,
                    flag,
                    reason,
                });
            }
        })
    } else {
        // If there are leftovers (pet new --name Mochi --kind blob --animalKind dog)
        // After '--kind blob', this else-branch is entered
        // That leaves the leftover ('--animalKind dog') hanging

        if let Some(flag) = args.peek() {
            if flag == "--animalKind" {
                let flag = args.next().unwrap();
                let _ = args.next();

                let reason = format!(
                    "Flag '--animalKind' can only be used when '--kind' is 'animal', but got '{kind}'!"
                );

                return Err(ParserError::InvalidArgument {
                    argument: flag,
                    flag: "--kind".into(),
                    reason,
                });
            }
        }

        None
    };

    Ok(Command::New {
        name,
        pet_type,
        animal_kind,
    })
}

fn parse_view(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> Result<Command, ParserError> {
    // Initial: (pet) (view) (--name <name>)?

    let name = if args.peek().is_some() {
        let (_, name) = parse_kv(args, "--name")?;
        Some(name)
    } else {
        None
    };

    Ok(Command::View { name })
}

fn parse_feed(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> Result<Command, ParserError> {
    // Initial: (pet) (feed) --petName <name> --itemName <name>

    let (_, pet_name) = parse_kv(args, "--petName")?;
    let (_, item_name) = parse_kv(args, "--itemName")?;

    Ok(Command::Feed {
        name: pet_name,
        item: item_name,
    })
}

fn parse_play(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> Result<Command, ParserError> {
    // Initial: (pet) (play) (--name <name>)?

    let name = if args.peek().is_some() {
        let (_, name) = parse_kv(args, "--name")?;
        Some(name)
    } else {
        None
    };

    Ok(Command::Play { name })
}

fn parse_sleep(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> Result<Command, ParserError> {
    // Initial: (pet) (sleep) (--name <name>)?

    let name = if args.peek().is_some() {
        let (_, name) = parse_kv(args, "--name")?;
        Some(name)
    } else {
        None
    };

    Ok(Command::Sleep { name })
}

fn parse_rename(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> Result<Command, ParserError> {
    // Initial: (pet) (rename) --petName <name> --newName <name>

    let (_, name) = parse_kv(args, "--petName")?;
    let (_, new_name) = parse_kv(args, "--newName")?;

    Ok(Command::Rename { name, new_name })
}

fn parse_disown(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> Result<Command, ParserError> {
    // Initial: (pet) (disown) (--name <name>)?

    let name = if args.peek().is_some() {
        let (_, name) = parse_kv(args, "--name")?;
        Some(name)
    } else {
        None
    };

    Ok(Command::Disown { name })
}

fn parse_shop(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> Result<Command, ParserError> {
    // Initial: (pet) (shop) --name <name> (--quantity <amount>)?

    let (_, name) = parse_kv(args, "--name")?;

    let quantity = if args.peek().is_some() {
        let (flag, value) = parse_kv(args, "--quantity")?;

        let reason = format!("Expected an integer, but got '{value}'!");
        let quantity = value
            .parse::<i32>()
            .map_err(|_| ParserError::InvalidArgument {
                argument: value.clone(),
                flag,
                reason,
            })?;

        Some(quantity)
    } else {
        None
    };

    Ok(Command::Shop { name, quantity })
}

// Primary Helper
fn parse_kv(
    arguments: &mut impl Iterator<Item = String>,
    expected: &str,
) -> Result<(String, String), ParserError> {
    let flag = arguments
        .next()
        .ok_or(ParserError::MissingFlag(expected.into()))?;

    if !flag.starts_with("--") {
        return Err(ParserError::InvalidFlag {
            got: flag,
            expected: expected.into(),
        });
    }

    if flag != expected {
        return Err(ParserError::UnknownFlag {
            got: flag,
            expected: expected.into(),
        });
    }

    let value = arguments
        .next()
        .ok_or(ParserError::MissingArgument(flag.clone()))?;

    Ok((flag, value))
}
