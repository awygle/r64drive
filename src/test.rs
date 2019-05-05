use super::*;

pub struct R64DriveTest {}

impl<'a, 'b> R64Drive<'a, 'b> for R64DriveTest {
    type Result = Result<(), ()>;
    fn get_version(&'a self) -> Self::Result {
        Ok(())
    }
}

impl R64DriveTest {
    pub fn new() -> R64DriveTest {
        R64DriveTest {}
    }
}

impl Default for R64DriveTest {
    fn default() -> Self {
        Self::new()
    }
}
