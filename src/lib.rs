pub mod consts;
pub mod ftdi;
pub mod test;

use num::FromPrimitive;
use std::fmt;
#[macro_use]
extern crate num_derive;

#[derive(Copy, Clone, Debug)]
pub enum R64DriveError<T> {
    InvalidCompletion(u32),
    InvalidMagic(u32),
    NativeError(T),
}
use R64DriveError::*;

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Command {
    LoadFromPC = 0x20,
    DumpToPC = 0x30,
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

pub trait R64Driver {
    type Error;
    fn send_u32(&self, val: u32) -> Result<usize, Self::Error>;
    fn recv_u32(&self) -> Result<u32, Self::Error>;

    fn send_u32_slice(&self, slice: &[u32]) -> Result<usize, Self::Error> {
        let mut result = 0;
        for &val in slice {
            result += self.send_u32(val)?;
        }
        Ok(result)
    }

    fn recv_u32_slice(&self, len: usize) -> Result<Vec<u32>, Self::Error> {
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(self.recv_u32()?);
        }
        Ok(result)
    }
}

pub struct R64Drive<'a, T: R64Driver> {
    driver: &'a T,
}

impl<'a, T: R64Driver> R64Drive<'a, T>
where
    R64DriveError<<T as R64Driver>::Error>: From<<T as R64Driver>::Error>,
{
    pub fn new(driver: &T) -> R64Drive<T> {
        R64Drive { driver }
    }

    fn send_cmd(
        &self,
        cmd_id: Command,
        args: &[u32],
        expected_len: usize,
        verify: bool,
    ) -> Result<Vec<u32>, R64DriveError<T::Error>> {
        let cmd_hdr = ((cmd_id as u32) << 24) | consts::COMMAND;
        self.driver.send_u32(cmd_hdr)?;

        self.driver.send_u32_slice(args)?;

        let response = self.driver.recv_u32_slice(expected_len)?;

        if verify {
            let completion_pkt = (cmd_id as u32) | consts::COMPARE;
            let resp_u32 = self.driver.recv_u32()?;
            if resp_u32 != completion_pkt {
                return Err(InvalidCompletion(resp_u32));
            }
        }

        Ok(response)
    }

    pub fn get_version(
        &self,
    ) -> Result<(HardwareVariant, FirmwareVersion), R64DriveError<T::Error>> {
        let response = self.send_cmd(Command::VersionRequest, &[], 2, true)?;
        if response[1] != consts::MAGIC {
            Err(InvalidMagic(response[1]))?;
        }

        let variant =
            HardwareVariant::from_u32(response[0] >> 16).unwrap_or(HardwareVariant::Unexpected);
        Ok((variant, FirmwareVersion(response[0] as u16)))
    }

    pub fn set_save_type(&self, save_type: SaveType) -> Result<(), R64DriveError<T::Error>> {
        self.send_cmd(Command::SetSaveType, &[save_type as u32], 0, true)
            .map(|_| ())
    }

    pub fn set_cic_type(&self, cic_type: CICType) -> Result<(), R64DriveError<T::Error>> {
        self.send_cmd(Command::SetCICType, &[cic_type as u32 | consts::OVERRIDE_CIC], 0, true)
            .map(|_| ())
    }

    pub fn set_ci_extended(&self, enable: bool) -> Result<(), R64DriveError<T::Error>> {
        self.send_cmd(Command::SetCIExtended, &[enable as u32], 0, true)
            .map(|_| ())
    }

    pub fn load_from_pc(
        &self,
        offset: u32,
        bank: BankIndex,
        data: &[u32],
    ) -> Result<(), R64DriveError<T::Error>> {
        // TODO: Upload 8MB at a time instead of asserting on the length
        assert!(data.len() <= consts::MAX_TRANSFER_SIZE);

        let mut args: Vec<u32> = Vec::with_capacity(data.len() + 2);
        args.push(offset);
        args.push((bank as u32) << 24 | (data.len() * 4) as u32);
        args.extend(data);
        self.send_cmd(Command::LoadFromPC, &args, 0, false).map(|_| ())
    }

    pub fn dump_to_pc(
        &self,
        offset: u32,
        bank: BankIndex,
        len: u32,
    ) -> Result<Vec<u32>, R64DriveError<T::Error>> {
        // TODO: Download 8MB at a time instead of asserting on the length
        assert!((len as usize) <= consts::MAX_TRANSFER_SIZE);

        self.send_cmd(
            Command::DumpToPC,
            &[offset, (bank as u32) << 24 | len],
            (len / 4) as usize,
            false,
        )
    }
}
