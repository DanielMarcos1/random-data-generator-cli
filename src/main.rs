use clap::{arg, command, Command};
use generator::TestOutputHandler;

mod generator;

fn main() {
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
        Some(("name", _generate)) => {
            let output_handler = TestOutputHandler;
            generator::generate_name(output_handler);
        }
        Some(("address", _generate)) => {
            let output_handler = TestOutputHandler;
            generator::generate_address(output_handler);
        }
        Some(("phone", _generate)) => {
            let output_handler = TestOutputHandler;
            generator::generate_phone(output_handler);
        }
        Some(("fullhuman", _generate)) => {
            let output_handler = TestOutputHandler;
            generator::generate_fullhuman(output_handler);
        }
        _ => {}
    }
}

