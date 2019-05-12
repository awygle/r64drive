#[cfg(test)]
mod tests {
    use r64drive::*;
    #[test]
    fn test_get_version() {
        let driver = test::R64DriverTest::new();
        let r64d = R64Drive::new(&driver);
        let (variant, version) = r64d.get_version().unwrap();
        assert!(variant == 0x4200);
        assert!(version == 0x00CD);
    }
}
