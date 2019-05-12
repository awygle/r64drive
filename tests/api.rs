#[cfg(test)]
mod tests {
    use r64drive::*;
    #[test]
    fn test_get_version() {
        let driver = test::R64DriverTest::new();
        let r64d = R64Drive::new(&driver);
        let (variant, version) = r64d.get_version().unwrap();
        assert!(variant != HardwareVariant::Unexpected);
        assert!(version.into_inner() == 205);
    }

    #[test]
    fn test_set_save_type() {
        let driver = test::R64DriverTest::new();
        let r64d = R64Drive::new(&driver);
        r64d.set_save_type(SaveType::None).unwrap();
    }

    #[test]
    fn test_set_cic_type() {
        let driver = test::R64DriverTest::new();
        let r64d = R64Drive::new(&driver);
        r64d.set_cic_type(CICType::CIC6101).unwrap();
    }
}
