pub enum EmulationEvents {
    DEBUG = 0b_0000_0001,
    VSYNC = 0b_0000_0010,
    BOOTROMDISABLED = 0b_0000_0100,
}

pub struct EmulationTime {
    pub machine_cycles: u64,
}

impl EmulationTime {
    pub fn new() -> Self {
        EmulationTime { machine_cycles: 0 }
    }
}

impl From<u64> for EmulationTime {
    fn from(machine_cycles: u64) -> Self {
        EmulationTime { machine_cycles }
    }
}
