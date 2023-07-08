use super::emulation::time::EmulationEvents;

pub enum Key {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

pub trait Callbacks {
    fn debug_opcode(&mut self);
    fn bootrom_disabled(&mut self);
    fn trigger_emulation_events(&mut self, event: &[EmulationEvents]);
}

pub trait CoreContext {
    fn callbacks(&mut self) -> Option<&mut dyn Callbacks>;
}
