use super::*;

pub struct R64DriveTest {}

impl<'a, 'b> R64Drive<'a, 'b> for R64DriveTest {
    type Result = Result<(), ()>;
    fn send_cmd(&'a self, _cmd_id: Commands, _args: &'b [u32]) -> Self::Result {
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
