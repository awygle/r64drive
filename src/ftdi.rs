use super::*;
use byteorder::{BigEndian, ByteOrder};
use safe_ftdi as ftdi;

pub struct R64DriveFtdi<'a> {
    context: ftdi::Context,
    _dummy: std::marker::PhantomData<&'a ()>,
}

impl<'a, 'b> R64Drive<'a, 'b> for R64DriveFtdi<'a> {
    type Result = ftdi::Result<'a, &'a [u32]>;
    fn get_version(&'a self) -> Self::Result {
        self.send_cmd(Commands::VersionRequest, &[])
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

    fn send_cmd(&'a self, cmd_id: Commands, args: &[u32]) -> ftdi::Result<'a, &'a [u32]> {
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

impl<'a> Default for R64DriveFtdi<'a> {
    fn default() -> Self {
        Self::new()
    }
}
