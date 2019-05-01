
pub enum Commands {
    VersionRequest = 0x80,
}

pub struct R64Drive {
    
}

impl R64Drive {
    pub fn send_cmd(&self, cmd_id: Commands, args: &[u32]) {
        
    }
    
    pub fn new() -> R64Drive {
        R64Drive { }
    }
}

