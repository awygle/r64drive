use super::*;
use std::cell::RefCell;

#[derive(Copy, Clone, Debug)]
enum State {
    Idle,
    VersionRequest,
    SetSaveType,
    Finished,
}

#[derive(Copy, Clone, Debug)]
struct R64DriveTest {
    state: State,
    command: Command,
}

impl Default for R64DriveTest {
    fn default() -> Self {
        R64DriveTest {
            state: State::Idle,
            command: Command::Unexpected,
        }
    }
}

impl R64DriveTest {
    fn recv_u32(&mut self, val: u32) -> Result<usize, (&'static str, u32)> {
        match self.state {
            State::Idle => match Command::from_u32(val >> 24).unwrap_or(Command::Unexpected) {
                Command::VersionRequest => {
                    self.state = State::VersionRequest;
                    self.command = Command::VersionRequest;
                    Ok(4)
                }
                Command::SetSaveType => {
                    self.state = State::SetSaveType;
                    self.command = Command::SetSaveType;
                    Ok(0)
                }
                _ => Err(("invalid command in state Idle", val)),
            },
            State::SetSaveType => {
                self.state = State::Finished;
                Ok(0)
            }
            State::VersionRequest => Err(("invalid packet in state VersionRequest", val)),
            State::Finished => Err(("invalid packet in state Finished", val)),
        }
    }

    fn send_u32(&mut self) -> Result<u32, (&'static str, u32)> {
        match self.state {
            State::Idle => Err(("unexpected read in state Idle", 0)),
            State::VersionRequest => {
                self.state = State::Finished;
                Ok(0x4200_00CD)
            }
            State::SetSaveType => Err(("unexpected read in state SetSaveType", 0)),
            State::Finished => Ok(0x43_4D_50_00u32 | self.command as u32),
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
                state: State::Idle,
                command: Command::Unexpected,
            }),
        }
    }
}

impl Default for R64DriverTest {
    fn default() -> Self {
        Self::new()
    }
}
