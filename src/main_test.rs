#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_name() {
        // Call the function and check if the generated name is not empty
        let name = generate_name();
        assert!(!name.is_empty(), "Generated name should not be empty");
    }

    #[test]
    fn test_generate_address() {
        // Call the function and check if the generated address is not empty
        let address = generate_address();
        assert!(
            !address.is_empty(),
            "Generated address should not be empty"
        );
    }

    #[test]
    fn test_generate_phone() {
        // Call the function and check if the generated phone number is not empty
        let phone = generate_phone();
        assert!(
            !phone.is_empty(),
            "Generated phone number should not be empty"
        );
    }

    #[test]
    fn test_generate_fullhuman() {
        // Call the function and check if the generated name, phone, address, job, and internet info are not empty
        let (name, phone, address, job, internet) = generate_fullhuman();
        assert!(!name.is_empty(), "Generated name should not be empty");
        assert!(
            !phone.is_empty(),
            "Generated phone number should not be empty"
        );
        assert!(
            !address.is_empty(),
            "Generated address should not be empty"
        );
        assert!(!job.is_empty(), "Generated job info should not be empty");
        assert!(
            !internet.is_empty(),
            "Generated internet info should not be empty"
        );
    }
}
