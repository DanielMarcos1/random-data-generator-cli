use fake::faker::address::en::{BuildingNumber, CityPrefix, CountryName, StateName, ZipCode};
use fake::faker::internet::en::{Password, SafeEmail, Username, IP};
use fake::faker::job::en::{Field, Seniority, Title};
use fake::faker::{address::en::CityName, name::en::Name, phone_number::en::PhoneNumber};
use fake::Fake;

// Aqui vai uma gambiarra para o compilador não pegar o println como tipo () e sim como uma string
// facilitando os testes unitários de serem realizados
trait OutputHandler {
    fn handle_output(&self, output: String);
}


impl OutputHandler for () {
    fn handle_output(&self, output: String) {
        println!("{}", output);
    }
}


pub struct TestOutputHandler;

impl OutputHandler for TestOutputHandler {
    fn handle_output(&self, output: String) {
        println!("{}", output);
    }
}

// Main logic
pub fn generate_fullhuman<O>(output_handler: O)
where
    O: OutputHandler,
{
    // ... (unchanged)
    let _name: String = Name().fake();
    let _phone: String = PhoneNumber().fake();
    let _address: String = format!(
        "{} {} {} {} {} {}",
        StateName().fake::<String>(),
        CityName().fake::<String>(),
        CityPrefix().fake::<String>(),
        CountryName().fake::<String>(),
        BuildingNumber().fake::<String>(),
        ZipCode().fake::<String>()
    );

    let _job: String = format!(
        "{} {} {}",
        Field().fake::<String>(),
        Seniority().fake::<String>(),
        Title().fake::<String>()
    );

    let _internet: String = format!(
        "{} {} {} {}",
        IP().fake::<String>(),
        Username().fake::<String>(),
        SafeEmail().fake::<String>(),
        Password(8..12).fake::<String>(),
    );

    let output = format!(
        "Human generated!\n
              Name: {}\n
              Phone: {}\n
              Address: {}\n
              Job: {}\n
              Internet data: {}\n",
        _name, _phone, _address, _job, _internet
    );

    output_handler.handle_output(output);
}


pub fn generate_name<O>(output_handler: O)
where
    O: OutputHandler,
{
    let name: String = Name().fake();
    let output = format!("Value for name: {}", name);
    output_handler.handle_output(output);
}

pub fn generate_address<O>(output_handler: O)
where
    O: OutputHandler,
{
    let address: String = CityName().fake();
    let output = format!("Value for address: {}", address);
    output_handler.handle_output(output);
}

pub fn generate_phone<O>(output_handler: O)
where
    O: OutputHandler,
{
    let phone: String = PhoneNumber().fake();
    let output = format!("Value for phone: {}", phone);
    output_handler.handle_output(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_name() {
        let output_handler = TestOutputHandler;
        generate_name(output_handler);
    }

    #[test]
    fn test_generate_address() {
        let output_handler = TestOutputHandler;
        generate_address(output_handler);
    }

    #[test]
    fn test_generate_phone() {
        let output_handler = TestOutputHandler;
        generate_phone(output_handler);
    }

}

