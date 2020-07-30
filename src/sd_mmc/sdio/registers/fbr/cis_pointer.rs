use crate::sd_mmc::sdio::registers::registers::Register;
use bit_field::BitField;

/// Address pointer to function CIS (3 bytes, LSB first
pub struct CisPointerRegister {
    pub val: u32
}

impl CisPointerRegister {
    pub fn set_lsb(&mut self, val: u8) {
        self.val.set_bits(16..23, val as u32);
    }

    pub fn set_mid(&mut self, val: u8) {
        self.val.set_bits(8..15, val as u32);
    }

    pub fn set_msb(&mut self, val: u8) {
        self.val.set_bits(0..7, val as u32);
    }
}

impl Register<u32> for CisPointerRegister {
    fn value(&self) -> u32 {
        self.val
    }

    fn address() -> u8 {
        0x9
    }
}