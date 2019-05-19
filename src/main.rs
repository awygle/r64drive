use byteorder::{BigEndian, ByteOrder};
use r64drive::ftdi::R64DriveFtdi;
use r64drive::{BankIndex, CICType, R64Drive, R64DriveError};
use std::{env, fmt, fs, io, mem};

enum Error {
    Driver,
    Io,
    NotN64,
    RomSize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Error::Driver => "Generic driver error",
            Error::Io => "Unable to read file",
            Error::NotN64 => "Provided file is not an N64 ROM",
            Error::RomSize => "Provided file is too small for N64",
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {{ {} }}", self)
    }
}

impl From<io::Error> for Error {
    fn from(_error: io::Error) -> Self {
        Error::Io
    }
}

impl<T> From<R64DriveError<T>> for Error {
    fn from(_error: R64DriveError<T>) -> Self {
        Error::Driver
    }
}

fn main() -> Result<(), Error> {
    let driver = R64DriveFtdi::new();
    let r64d = R64Drive::new(&driver);
    let (variant, version) = r64d.get_version()?;
    println!("variant: {:?}, version: {}", variant, version);

    r64d.set_cic_type(CICType::CIC6102)?;

    let mut args = env::args().skip(1);
    if let Some(filepath) = args.next() {
        let f = fs::read(filepath)?;

        // Basic sanity checks; ensure this is an N64 ROM, non-byte-swapped
        if f.len() < 0x10_1000 {
            Err(Error::RomSize)?;
        }
        if BigEndian::read_u32(&f) != 0x8037_1240 {
            Err(Error::NotN64)?;
        }

        // Read ROM file into Vec<u32>
        let size = f.len() / mem::size_of::<u32>();
        let mut rom = Vec::with_capacity(size);
        rom.resize(size, 0);

        println!("Uploading {} bytes...", f.len());
        BigEndian::read_u32_into(&f, &mut rom);
        r64d.load_from_pc(0, BankIndex::CartridgeROM, &rom)?;
        println!("Done!");
    }

    Ok(())
}
