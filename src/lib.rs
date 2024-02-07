use clap::{arg, command, Command};

mod generator;

type Result<T> = std::result::Result<T, String>;

pub fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}

pub fn run() -> Result<()> {
    let matches = Command::new("Fake Data Generator")
        .version("0.1.3")
        .author("Daniel Marcos")
        .about("Generates random names, addresses, and phone numbers")
        .arg(arg!([generate] "Generates the value"))
        .subcommand(
            command!("name")
                .about("Generates a random name and prints it on the console"),
        )
        .subcommand(
            command!("address")
                .about("Generates a random address and prints it on the console"),
        )
        .subcommand(
            command!("phone")
                .about("Generates a random phone number and prints it on the console"),
        )
        .subcommand(
            command!("fullhuman")
                .about("Generates a random human and prints it on the console"),
        )
        .get_matches();

        match matches.subcommand() {
            Some(("name", _generate)) => Ok(generator::generate_name(())),
            Some(("address", _generate)) => Ok(generator::generate_address(())),
            Some(("phone", _generate)) => Ok(generator::generate_phone(())),
            Some(("fullhuman", _generate)) => Ok(generator::generate_fullhuman(())),
            _ => Err(format!("Unrecognized command")),
        }
}