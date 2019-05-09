use super::*;
use byteorder::{BigEndian, ByteOrder};
use safe_ftdi as ftdi;

pub struct R64DriveFtdi<'a> {
    context: ftdi::Context,
    _dummy: std::marker::PhantomData<&'a ()>,
}

impl<'a> R64Driver<'a> for R64DriveFtdi<'a> {
    type Error = ftdi::error::Error<'a>;
    fn send_cmd(&'a self, cmd_id: Commands, args: &[u32]) -> Result<Vec<u32>, Self::Error> {
        let mut buf: Vec<u8> = Vec::with_capacity((args.len() + 1) * 4);
        let cmd_hdr = ((cmd_id as u32) << 24) | 0x43_4D_44u32;
        BigEndian::write_u32(&mut buf, cmd_hdr);
        BigEndian::write_u32_into(args, &mut buf[4..]);

        self.context.write_data(&buf)?;

        let mut response = Vec::new();
        let completion_pkt = (cmd_id as u32) | 0x43_4D_50u32;

        loop {
            let mut resp = [0u8; 4];

            self.context.read_data(&mut resp)?;
            let resp_u32 = BigEndian::read_u32(&resp);

            response.push(resp_u32);

            // TODO handle commands which don't return this value
            if resp_u32 == completion_pkt {
                return Ok(response);
            }
        }
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
