pub mod cli;
pub mod common;
pub mod data;
pub mod errors;
pub mod inventory;
pub mod pet_collection;
pub mod pets;

fn main() -> std::process::ExitCode {
    match cli::app::run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            std::process::ExitCode::FAILURE
        }
    }
}
