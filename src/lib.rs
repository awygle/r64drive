pub mod ftdi;
pub mod test;

#[derive(Copy, Clone, Debug)]
pub enum Commands {
    VersionRequest = 0x80,
}

pub trait R64Driver<'a> {
    type Error;
    fn send_u32(&'a self, val: u32) -> Result<usize, Self::Error>;
    fn recv_u32(&'a self) -> Result<u32, Self::Error>;
}

pub struct R64Drive<'a, T: R64Driver<'a>> {
    driver: &'a T,
}

impl<'a, T: R64Driver<'a>> R64Drive<'a, T> {
    pub fn new(driver: &'a T) -> R64Drive<T> {
        R64Drive { driver }
    }

    fn send_cmd(&'a self, cmd_id: Commands, args: &[u32]) -> Result<Vec<u32>, T::Error> {
        let cmd_hdr = ((cmd_id as u32) << 24) | 0x43_4D_44u32;
        self.driver.send_u32(cmd_hdr)?;

        for arg in args {
            self.driver.send_u32(*arg)?;
        }

        let mut response = Vec::new();
        let completion_pkt = (cmd_id as u32) | 0x43_4D_50_00u32;

        loop {
            let resp_u32 = self.driver.recv_u32()?;

            response.push(resp_u32);

            // TODO handle commands which don't return this value
            if resp_u32 == completion_pkt {
                return Ok(response);
            }
        }
    }

    pub fn get_version(&'a self) -> Result<(u16, u16), T::Error> {
        let response = self.send_cmd(Commands::VersionRequest, &[])?[0];
        Ok(((response >> 16) as u16, response as u16))
    }
}
