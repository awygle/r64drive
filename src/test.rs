use super::*;
use byteorder::{BigEndian, ByteOrder};

pub struct R64DriveTest {}

impl R64Drive for R64DriveTest {
    fn send_cmd(&self, cmd_id: Commands, args: &[u32]) {}
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
