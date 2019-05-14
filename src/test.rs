use super::*;
use std::cell::RefCell;

#[derive(Copy, Clone, Debug)]
enum State {
    Idle,
    LoadFromPCOffset,
    LoadFromPCMeta,
    LoadFromPCData,
    DumpToPCOffset,
    DumpToPCMeta,
    DumpToPCData,
    SetSaveType,
    SetCICType,
    SetCIExtended,
    VersionRequest,
    Finished,
}

#[derive(Copy, Clone, Debug)]
struct R64DriveTest {
    state: State,
    command: Command,
    data_len: u32,
}

impl Default for R64DriveTest {
    fn default() -> Self {
        R64DriveTest {
            state: State::Idle,
            command: Command::Unexpected,
            data_len: 0,
        }
    }
}

impl From<(&'static str, u32)> for R64DriveError<(&'static str, u32)> {
    fn from(err: (&'static str, u32)) -> Self {
        R64DriveError::NativeError(err)
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
                    Ok(4)
                }
                Command::SetCICType => {
                    self.state = State::SetCICType;
                    self.command = Command::SetCICType;
                    Ok(4)
                }
                Command::SetCIExtended => {
                    self.state = State::SetCIExtended;
                    self.command = Command::SetCIExtended;
                    Ok(4)
                }
                Command::LoadFromPC => {
                    self.state = State::LoadFromPCOffset;
                    self.command = Command::LoadFromPC;
                    Ok(4)
                }
                Command::DumpToPC => {
                    self.state = State::DumpToPCOffset;
                    self.command = Command::DumpToPC;
                    Ok(4)
                }
                Command::Unexpected => Err(("unexpected command in state Idle", val)),
            },
            State::SetSaveType => {
                self.state = State::Finished;
                Ok(4)
            }
            State::SetCICType => {
                self.state = State::Finished;
                Ok(4)
            }
            State::SetCIExtended => {
                self.state = State::Finished;
                Ok(4)
            }
            State::LoadFromPCOffset => {
                self.state = State::LoadFromPCMeta;
                Ok(4)
            }
            State::LoadFromPCMeta => {
                self.state = State::LoadFromPCData;
                self.data_len = (val & 0x00FF_FFFFu32) / 4;
                Ok(4)
            }
            State::LoadFromPCData => {
                self.data_len -= 1;
                if self.data_len == 0 {
                    self.state = State::Finished;
                }
                Ok(4)
            }
            State::DumpToPCOffset => {
                self.state = State::DumpToPCMeta;
                Ok(4)
            }
            State::DumpToPCMeta => {
                self.state = State::DumpToPCData;
                self.data_len = (val & 0x00FF_FFFFu32) / 4;
                Ok(4)
            }
            State::DumpToPCData => Err(("invalid packet in state DumpToPCData", val)),
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
            State::SetCICType => Err(("unexpected read in state SetCICType", 0)),
            State::SetCIExtended => Err(("unexpected read in state SetCIExtended", 0)),
            State::LoadFromPCOffset => Err(("unexpected read in state LoadFromPCOffset", 0)),
            State::LoadFromPCMeta => Err(("unexpected read in state LoadFromPCMeta", 0)),
            State::LoadFromPCData => Err(("unexpected read in state LoadFromPCData", 0)),
            State::DumpToPCOffset => Err(("unexpected read in state DumpToPCOffset", 0)),
            State::DumpToPCMeta => Err(("unexpected read in state DumpToPCMeta", 0)),
            State::DumpToPCData => {
                self.data_len -= 1;
                if self.data_len == 0 {
                    self.state = State::Finished;
                }
                Ok(0)
            }
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
                data_len: 0,
            }),
        }
    }
}

impl Default for R64DriverTest {
    fn default() -> Self {
        Self::new()
    }
}
