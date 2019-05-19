/// Command magic number "_CMD"
pub const COMMAND: u32 = 0x43_4D_44;

/// Compare magic number "CMP_"
pub const COMPARE: u32 = 0x43_4D_50_00;

/// Device magic number: "UDEV"
pub const MAGIC: u32 = 0x55_44_45_56;

/// Override default CIC type
pub const OVERRIDE_CIC: u32 = 0x8000_0000;

/// Maximum data transfer size (in bytes)
pub const MAX_TRANSFER_SIZE: usize = 8 * 1024 * 1024 / 4;
