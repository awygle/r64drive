pub mod ftdi;
pub mod test;

pub enum Commands {
    VersionRequest = 0x80,
}

pub trait R64Drive<'a> {
    type Error;
    fn get_version(&'a self) -> Result<&[u32], Self::Error>;
}
