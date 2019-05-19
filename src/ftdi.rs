use super::*;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use ftdi::mpsse::MpsseMode;
use safe_ftdi as ftdi;

pub struct R64DriveFtdi {
    context: ftdi::Context,
}

impl R64Driver for R64DriveFtdi {
    type Error = ftdi::error::Error;
    fn send_u32(&self, val: u32) -> Result<usize, Self::Error> {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, val);
        self.context.write_data(&buf).map(|x| x as usize)
    }

    fn recv_u32(&self) -> Result<u32, Self::Error> {
        let mut resp = [0u8; 4];
        self.context.read_data(&mut resp)?;
        Ok(BigEndian::read_u32(&resp))
    }

    fn send_u32_slice(&self, slice: &[u32]) -> Result<usize, Self::Error> {
        let mut buf = Vec::with_capacity(4 * slice.len());
        let buf_wtr = &mut buf;
        for &val in slice {
            buf_wtr
                .write_u32::<BigEndian>(val)
                .expect("Error writing to vector");
        }
        self.context.write_data(&buf).map(|x| x as usize)
    }

    fn recv_u32_slice(&self, len: usize) -> Result<Vec<u32>, Self::Error> {
        let mut buf = Vec::with_capacity(4 * len);
        self.context.read_data(&mut buf)?;
        let mut buf_rdr: &[u8] = &mut buf;
        let mut result = Vec::with_capacity(len);
        while let Ok(val) = buf_rdr.read_u32::<BigEndian>() {
            result.push(val);
        }
        Ok(result)
    }
}

impl R64DriveFtdi {
    pub fn new() -> R64DriveFtdi {
        // TODO take a hardware version or VID/PID
        let mut result = R64DriveFtdi {
            context: ftdi::Context::new().unwrap(),
        };
        result.context.open(0x0403, 0x6014).unwrap();

        // Initialize/reset hardware (HW2 only?)
        result
            .context
            .set_bitmode(0xFF, MpsseMode::BITMODE_RESET)
            .unwrap();
        result
            .context
            .set_bitmode(0xFF, MpsseMode::BITMODE_SYNCFF)
            .unwrap();

        result.context.purge_usb_buffers().unwrap();

        result
    }
}

impl Default for R64DriveFtdi {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ftdi::error::Error> for R64DriveError<ftdi::error::Error> {
    fn from(err: ftdi::error::Error) -> Self {
        R64DriveError::NativeError(err)
    }
}
