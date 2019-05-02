pub mod ftdi;
pub mod test;

pub enum Commands {
    VersionRequest = 0x80,
}

pub trait R64Drive {
    fn send_cmd(&self, cmd_id: Commands, args: &[u32]);
}
