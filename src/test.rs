use super::*;

pub struct R64DriveTest {}

impl<'a> R64Driver<'a> for R64DriveTest {
    type Error = ();
    fn send_cmd(&self, cmd_id: Commands, _args: &[u32]) -> Result<&[u32], Self::Error> {
        match cmd_id {
            Commands::VersionRequest => Ok(&[0x4200_00CD]),
        }
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
