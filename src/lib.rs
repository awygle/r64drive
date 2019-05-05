pub mod ftdi;
pub mod test;

pub enum Commands {
    VersionRequest = 0x80,
}

pub trait R64Drive<'a, 'b> {
    type Result;
    fn send_cmd(&'a self, cmd_id: Commands, args: &'b [u32]) -> Self::Result;
}
