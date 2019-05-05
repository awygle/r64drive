pub mod ftdi;
pub mod test;

pub enum Commands {
    VersionRequest = 0x80,
}

pub trait R64Drive<'a, 'b> {
    type Result;
    fn get_version(&'a self) -> Self::Result;
}
