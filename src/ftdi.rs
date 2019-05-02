use super::*;
use byteorder::{BigEndian, ByteOrder};
use safe_ftdi as ftdi;

pub struct R64DriveFtdi {
    context: ftdi::Context,
}

impl R64Drive for R64DriveFtdi {
    fn send_cmd(&self, cmd_id: Commands, args: &[u32]) {
        let mut buf: Vec<u8> = Vec::with_capacity((args.len() + 1) * 4);
        BigEndian::write_u32(&mut buf, cmd_id as u32);
        BigEndian::write_u32_into(args, &mut buf[4..]);
        self.context.write_data(&buf).unwrap();
    }
}

impl R64DriveFtdi {
    pub fn new() -> R64DriveFtdi {
        // TODO take a hardware version or VID/PID
        let mut result = R64DriveFtdi {
            context: ftdi::Context::new().unwrap(),
        };
        result.context.open(0x0403, 0x6014).unwrap();
        result
    }
}

impl Default for R64DriveFtdi {
    fn default() -> Self {
        Self::new()
    }
}
