# pet

> a tiny virtual pet CLI written in Rust because apparently taking care of imaginary text is now a terminal responsibility!!!

i made this mainly to practice working with:

- error handling (especially with `Result<T, E>`)

## About

`pet` is a command-line virtual pet simulator written in `Rust` . . . (AS ALWAYS??)

create your own pets, check up on them, feed them, play with them, and let them rest--all from your terminal!!
(how convenient for.. ppl who never leave their chair coding?? all day????)

this project combines a little virtual-pet gameplay with practical systems such as command parsing, data persistence, inventory management, and error handling!! (as well as `Rust`'s hell-level gimmicks..)

i made this mainly to practice working with, uhm:

- `trait`-based `polymorphism`
- `command-line` `argument` parsing
- data `serialization` and persistence
- inventory and `collection` management
- `error` handling
- modular application architecture
- `Rust`'s `ownership` and `borrowing` system (LIKE ALWAYS. this is a canon event for all Rust coders)

## Features

- multiple pet types: `animal`s (`dog` + `cat`), `blob`s, and `robot`s
- animal varieties, including dogs and cats
- `ASCII` art models that reflect your pets' `state`s
- pet profiles with stats such as HP, hunger, happiness, and their `gimmick` stat (such as `energy` for animal types)
- `feed` pets using items from your inventory
- `play` with your pets and let them `sleep`
- `rename` pets or `disown` them (idk why i made this.. oh how i pity them....)
- manage an inventory and create your own `food` `item`s
- persistent pet and inventory data between runs
- a `command-line` interface for basically managing everything.!!
  
## Quick Start

### Requirements

- `Rust`
- `Cargo`

### Build from Source

clone the repository:

```bash
git clone https://github.com/jayywashere/pet
cd pet
```

build the release executable:

```bash
cargo build --release
# idk about that "--release" tag's significance, so you decide
```

### Usage

ideally, check all the available commands first with:

```bash
pet help
```

#### Create a Pet

create a dog named `Mochi`: (quick fun fact: i named all my dogs after food)

```bash
pet new --name Mochi --kind animal --animalKind dog
```

you can also create other kinds of pets, such as blobs and robots!!

#### View your Pets

view every pet:

```bash
pet view
```

view a specific pet:

```bash
pet view --name Mochi
# replace "Mochi" with the actual name of the desired pet
```

each profile displays the pet's information, current state, stats, and ASCII art model!>!>

#### Take Care of your Pets

feed a pet using an inventory item:

```bash
pet feed --name Mochi --item apple
# do make sure you have an "apple" in your inventory
# and to do that, you can enter "pet inventory" to view it!
```

play with your pets:

```bash
pet play # this applies to all your pets
pet play --name Mochi # this applies to a specific pet that's named "Mochi"
```

or let them sleep:

```bash
pet sleep # this applies to all your pets, too
pet sleep --name Mochi # this applies to a specific pet that's named "Mochi", too
```

#### Manage your Pets

rename a pet:

```bash
pet rename --name Mochi --newName Mochi2
# by the way, a reserved word is "all", and i also don't know why i made it reserved
# so.... yeah!! make sure it's not "all" and not a name that a different pet already owns
```

disown a specific pet: (awwh.)

```bash
pet disown # this disowns EVERY SINGLE PET YOU HAVE!!!! practice extreme CAUTION. (unless you're a maniac /j)
pet disown --name Mochi # this disowns.. only "Mochi"... how sad....... such a cruel world this is, no?.. .....
```

#### Manage your Inventory

view your inventory:

```bash
pet inventory
```

create and get that item:

```bash
pet shop --name apple
# you get a free item named "apple", so it's like.. you make your own thing
# it defaults to a "quantity" of 1, btw
```

create an item and receive multiple duplicates of it (depending on quantity given):

```bash
pet shop --name apple --quantity 5
# you create an item named "apple" 5 times that you obtain
```

## Data Persistence

furthermore, your pets and inventory are saved locally, allowing them to persist between program runs. (I'M TELLING YOU. I SUFFERED IN CODING THE PERSISTENCE MECHANIC THE MOST.)

the application initializes its save data automatically when needed!! you don't need to run a separate setup command (i don't even got one.. so how??)

## Why I Made This

i lowk wanted to build smth more interesting than another basic CLI program while practicing Rust and working with a more structed app!! (don't make me remember how many times i've said this, but it remains the truth)

anyway!! a virtual pet simulator gave me an excuse to work with `trait`s, different pet `impl`ementations, `collection`s, inventories, command parsing, `serialization`, and persistent data . . .

AND, OF COURSE, ASCII PETS

## License

See [LICENSE](LICENSE).