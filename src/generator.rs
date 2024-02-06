use fake::faker::address::en::{BuildingNumber, CityPrefix, CountryName, StateName, ZipCode};
use fake::faker::internet::en::{Password, SafeEmail, Username, IP};
use fake::faker::job::en::{Field, Seniority, Title};
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

pub fn generate_fullhuman() {
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
}
