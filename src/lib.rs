pub mod ftdi;
pub mod test;

use num::FromPrimitive;
use std::fmt;
#[macro_use]
extern crate num_derive;

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Command {
    VersionRequest = 0x80,
    SetSaveType = 0x70,
    Unexpected,
}

#[derive(Copy, Clone, Debug)]
pub enum SaveType {
    None = 0,
    EEPROM4k = 1,
    EEPROM16k = 2,
    SRAM256k = 3,
    FlashRAM1M = 4,
    SRAM768k = 5,
    FlashRAM1MPkmn = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq)]
pub enum HardwareVariant {
    RevA = 0x4100,
    RevB = 0x4200,
    Unexpected,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FirmwareVersion(u16);

impl fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let major = self.0 / 100;
        let minor = self.0 % 100;
        write!(f, "{}.{}", major, minor)
    }
}

impl FirmwareVersion {
    pub fn into_inner(self) -> u16 {
        self.0
    }
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

    fn send_cmd(&'a self, cmd_id: Command, args: &[u32]) -> Result<Vec<u32>, T::Error> {
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

    pub fn get_version(&'a self) -> Result<(HardwareVariant, FirmwareVersion), T::Error> {
        let response = self.send_cmd(Command::VersionRequest, &[])?[0];
        let variant =
            HardwareVariant::from_u32(response >> 16).unwrap_or(HardwareVariant::Unexpected);
        Ok((variant, FirmwareVersion(response as u16)))
    }

    pub fn set_save_type(&'a self, save_type: SaveType) -> Result<(), T::Error> {
        self.send_cmd(Command::SetSaveType, &[save_type as u32])
            .map(|_| ())
    }
}
