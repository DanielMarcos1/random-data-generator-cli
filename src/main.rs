use clap::{arg, command, Command};

mod generator;
fn main() {
    let matches = Command::new("Fake Data Generator")
        .version("0.1.2")
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
            Some(("name", _generate)) => generator::generate_name(),
            Some(("address", _generate)) => generator::generate_address(),
            Some(("phone", _generate)) => generator::generate_phone(),
            Some(("fullhuman", _generate)) => generator::generate_fullhuman(),
            _ => {}
    }
}
