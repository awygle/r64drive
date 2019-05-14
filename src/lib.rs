pub mod ftdi;
pub mod test;

use num::FromPrimitive;
use std::fmt;
#[macro_use]
extern crate num_derive;

#[derive(Copy, Clone, Debug)]
pub enum R64DriveError<T> {
    InvalidCompletion(u32),
    NativeError(T),
}
use R64DriveError::*;

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Command {
    LoadFromPC = 0x20,
    SetSaveType = 0x70,
    SetCICType = 0x72,
    SetCIExtended = 0x74,
    VersionRequest = 0x80,
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

#[derive(Copy, Clone, Debug)]
pub enum CICType {
    CIC6101 = 0,
    CIC6102 = 1,
    CIC7101 = 2,
    CIC7102 = 3,
    CIC6103_7103 = 4,
    CIC6105_7105 = 5,
    CIC6106_7106 = 6,
    CIC5101 = 7,
}

#[derive(Copy, Clone, Debug)]
pub enum BankIndex {
    Invalid = 0,
    CartridgeROM = 1,
    SRAM256k = 2,
    SRAM768k = 3,
    FlashRAM = 4,
    FlashRAMPkmn = 5,
    EEPROM = 6,
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

impl<'a, T: R64Driver<'a>> R64Drive<'a, T>
where
    R64DriveError<<T as R64Driver<'a>>::Error>: From<<T as R64Driver<'a>>::Error>,
{
    pub fn new(driver: &'a T) -> R64Drive<T> {
        R64Drive { driver }
    }

    fn send_cmd(
        &'a self,
        cmd_id: Command,
        args: &[u32],
        expected_len: usize,
    ) -> Result<Vec<u32>, R64DriveError<T::Error>> {
        let cmd_hdr = ((cmd_id as u32) << 24) | 0x43_4D_44u32;
        self.driver.send_u32(cmd_hdr)?;

        for arg in args {
            self.driver.send_u32(*arg)?;
        }

        let mut response = Vec::new();
        let completion_pkt = (cmd_id as u32) | 0x43_4D_50_00u32;

        for _ in 0..expected_len {
            let resp_u32 = self.driver.recv_u32()?;

            response.push(resp_u32);
        }

        let resp_u32 = self.driver.recv_u32()?;
        if resp_u32 != completion_pkt {
            return Err(InvalidCompletion(resp_u32));
        }

        Ok(response)
    }

    pub fn get_version(
        &'a self,
    ) -> Result<(HardwareVariant, FirmwareVersion), R64DriveError<T::Error>> {
        let response = self.send_cmd(Command::VersionRequest, &[], 1)?[0];
        let variant =
            HardwareVariant::from_u32(response >> 16).unwrap_or(HardwareVariant::Unexpected);
        Ok((variant, FirmwareVersion(response as u16)))
    }

    pub fn set_save_type(&'a self, save_type: SaveType) -> Result<(), R64DriveError<T::Error>> {
        self.send_cmd(Command::SetSaveType, &[save_type as u32], 0)
            .map(|_| ())
    }

    pub fn set_cic_type(&'a self, cic_type: CICType) -> Result<(), R64DriveError<T::Error>> {
        self.send_cmd(Command::SetCICType, &[cic_type as u32], 0)
            .map(|_| ())
    }

    pub fn set_ci_extended(&'a self, enable: bool) -> Result<(), R64DriveError<T::Error>> {
        self.send_cmd(Command::SetCIExtended, &[enable as u32], 0)
            .map(|_| ())
    }

    pub fn load_from_pc(
        &'a self,
        offset: u32,
        bank: BankIndex,
        data: &[u32],
    ) -> Result<(), R64DriveError<T::Error>> {
        let mut args: Vec<u32> = Vec::with_capacity(data.len() + 2);
        args.push(offset);
        args.push((bank as u32) << 24 | data.len() as u32);
        args.extend(data);
        self.send_cmd(Command::LoadFromPC, &args, 0).map(|_| ())
    }
}
