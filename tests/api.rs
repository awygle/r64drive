#[cfg(test)]
mod tests {
    use r64drive::*;
    #[test]
    fn test_get_version() {
        let driver = test::R64DriveTest::new();
        let r64d = R64Drive::new(&driver);
        r64d.get_version().unwrap();
    }
}
