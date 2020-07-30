use crate::sd_mmc::sdio::registers::registers::Register;

pub struct ExtendedInterfaceCodeRegister {
    pub val: u8
}

impl Register for ExtendedInterfaceCodeRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x1
    }
}