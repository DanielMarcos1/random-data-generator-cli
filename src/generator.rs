use fake::faker::{address::en::CityName, name::en::Name, phone_number::en::PhoneNumber};
use fake::Fake;

pub fn generate_name() {
    let name: String = Name().fake();
    println!("Value for name: {}", name);
}

pub fn generate_address() {
    let address: String = CityName().fake();
    println!("Value for address: {}", address);
}

pub fn generate_phone() {
    let phone: String = PhoneNumber().fake();
    println!("Value for phone: {}", phone);
}
