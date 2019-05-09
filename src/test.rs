use super::*;

#[derive(Copy, Clone, Debug)]
enum States {
    Idle,
    VersionRequest,
    Finished,
}

pub struct R64DriveTest {
    state: std::cell::Cell<States>,
}

impl<'a> R64Driver<'a> for R64DriveTest {
    type Error = (&'static str, u32);
    fn send_u32(&'a self, val: u32) -> Result<usize, Self::Error> {
        match self.state.get() {
            States::Idle => match val >> 24 {
                0x80 => {
                    self.state.set(States::VersionRequest);
                    Ok(4)
                }
                _ => Err(("invalid command in state Idle", val)),
            },
            States::VersionRequest => Err(("invalid packet in state VersionRequest", val)),
            States::Finished => Err(("invalid packet in state Finished", val)),
        }
    }

    fn recv_u32(&'a self) -> Result<u32, Self::Error> {
        match self.state.get() {
            States::Idle => Err(("unexpected read in state Idle", 0)),
            States::VersionRequest => {
                self.state.set(States::Finished);
                Ok(0x4200_00CD)
            }
            States::Finished => Ok(0x43_4D_50_80u32),
        }
    }
}

impl R64DriveTest {
    pub fn new() -> R64DriveTest {
        R64DriveTest {
            state: std::cell::Cell::new(States::Idle),
        }
    }
}

impl Default for R64DriveTest {
    fn default() -> Self {
        Self::new()
    }
}
