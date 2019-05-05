#[cfg(test)]
mod tests {
    use r64drive::*;
    #[test]
    fn test_get_version() {
        let r64d = r64drive::test::R64DriveTest::new();
        r64d.get_version().unwrap();
    }
}
