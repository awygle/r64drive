use super::*;
use std::cell::RefCell;

#[derive(Copy, Clone, Debug)]
enum States {
    Idle,
    VersionRequest,
    Finished,
}

#[derive(Copy, Clone, Debug)]
struct R64DriveTest {
    state: States,
}

impl Default for R64DriveTest {
    fn default() -> Self {
        R64DriveTest {
            state: States::Idle,
        }
    }
}

impl R64DriveTest {
    fn recv_u32(&mut self, val: u32) -> Result<usize, (&'static str, u32)> {
        match self.state {
            States::Idle => match val >> 24 {
                0x80 => {
                    self.state = States::VersionRequest;
                    Ok(4)
                }
                _ => Err(("invalid command in state Idle", val)),
            },
            States::VersionRequest => Err(("invalid packet in state VersionRequest", val)),
            States::Finished => Err(("invalid packet in state Finished", val)),
        }
    }

    fn send_u32(&mut self) -> Result<u32, (&'static str, u32)> {
        match self.state {
            States::Idle => Err(("unexpected read in state Idle", 0)),
            States::VersionRequest => {
                self.state = States::Finished;
                Ok(0x4200_00CD)
            }
            States::Finished => Ok(0x43_4D_50_80u32),
        }
    }
}

pub struct R64DriverTest {
    mock: RefCell<R64DriveTest>,
}

impl<'a> R64Driver<'a> for R64DriverTest {
    type Error = (&'static str, u32);
    fn send_u32(&'a self, val: u32) -> Result<usize, Self::Error> {
        self.mock.borrow_mut().recv_u32(val)
    }

    fn recv_u32(&'a self) -> Result<u32, Self::Error> {
        self.mock.borrow_mut().send_u32()
    }
}

impl R64DriverTest {
    pub fn new() -> R64DriverTest {
        R64DriverTest {
            mock: RefCell::new(R64DriveTest {
                state: States::Idle,
            }),
        }
    }
}

impl Default for R64DriverTest {
    fn default() -> Self {
        Self::new()
    }
}
