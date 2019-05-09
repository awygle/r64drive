use super::*;
use byteorder::{BigEndian, ByteOrder};
use safe_ftdi as ftdi;

pub struct R64DriveFtdi<'a> {
    context: ftdi::Context,
    _dummy: std::marker::PhantomData<&'a ()>,
}

impl<'a> R64Driver<'a> for R64DriveFtdi<'a> {
    type Error = ftdi::error::Error<'a>;
    fn send_cmd(&'a self, cmd_id: Commands, args: &[u32]) -> Result<&[u32], Self::Error> {
        let mut buf: Vec<u8> = Vec::with_capacity((args.len() + 1) * 4);
        BigEndian::write_u32(&mut buf, cmd_id as u32);
        BigEndian::write_u32_into(args, &mut buf[4..]);

        let ftdi_result = self.context.write_data(&buf);

        if ftdi_result.is_err() {
            return Err(ftdi_result.unwrap_err());
        }

        // TODO return actual results
        Ok(&[])
    }
}

impl<'a> R64DriveFtdi<'a> {
    pub fn new() -> R64DriveFtdi<'a> {
        // TODO take a hardware version or VID/PID
        let mut result = R64DriveFtdi {
            context: ftdi::Context::new().unwrap(),
            _dummy: std::marker::PhantomData,
        };
        result.context.open(0x0403, 0x6014).unwrap();
        result
    }

}

impl<'a> Default for R64DriveFtdi<'a> {
    fn default() -> Self {
        Self::new()
    }
}
