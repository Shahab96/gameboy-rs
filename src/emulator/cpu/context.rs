use crate::emulator::core::CoreContext;

pub trait CpuContext: CoreContext {
    fn read_cycle(&mut self, addr: u16) -> u8;
    fn read_cycle_high(&mut self, addr: u8) -> u8 {
        self.read_cycle(0xff00 | (addr as u16))
    }
    // fn read_cycle_intr(&mut self, addr: u16) -> (InterruptLine, u8);
    fn write_cycle(&mut self, addr: u16, data: u8);
    fn write_cycle_high(&mut self, addr: u8, data: u8) {
        self.write_cycle(0xff00 | (addr as u16), data);
    }
    // fn write_cycle_intr(&mut self, addr: u16, data: u8) -> InterruptLine;
    fn tick_cycle(&mut self);
    fn has_interrupt(&self) -> bool;
    // fn ack_interrupt(&mut self, mask: InterruptLine);
}
