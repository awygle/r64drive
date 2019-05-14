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

    #[test]
    fn test_set_ci_extended() {
        let driver = test::R64DriverTest::new();
        let r64d = R64Drive::new(&driver);
        r64d.set_ci_extended(true).unwrap();
    }

    #[test]
    fn test_load_from_pc() {
        let driver = test::R64DriverTest::new();
        let r64d = R64Drive::new(&driver);
        let offset = 0u32;
        let index = BankIndex::CartridgeROM;
        let data = [0u32; 4096];
        r64d.load_from_pc(offset, index, &data).unwrap();
    }

    #[test]
    fn test_dump_to_pc() {
        let driver = test::R64DriverTest::new();
        let r64d = R64Drive::new(&driver);
        let offset = 0u32;
        let index = BankIndex::CartridgeROM;
        let len = 4096;
        let data = r64d.dump_to_pc(offset, index, len).unwrap();
        assert!(data.len() == (len / 4) as usize);
    }
}
