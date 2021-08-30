pub struct Mapper000{
    nPRGBanks: u8,
    nCHRBanks: u8,
}
impl Mapper000{
    pub fn new(nPRG: u8, nCHR: u8) -> Mapper000{
        return Mapper000{nPRGBanks: nPRG, nCHRBanks:nCHR}
    }

    pub fn cpu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool{
        if addr >= 0x8000 && addr <= 0xFFFF
        {
            *mapped_addr = (addr & (if self.nPRGBanks > 1 {0x7FFF} else {0x3FFF})) as u32;
            return true;
        }
        return false;
    }

    pub fn cpu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool{
        if addr >= 0x8000 && addr <= 0xFFFF
        {
            *mapped_addr = (addr & (if self.nPRGBanks > 1 {0x7FFF} else {0x3FFF})) as u32;
            return true;
        }
        return false;
    }



    pub fn ppu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool{
        if addr >= 0x0000 && addr <= 0x1FFF
        {
            *mapped_addr = addr as u32;
            return true;
        }
        return false;
    }

    pub fn ppu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool{
        if addr >= 0x0000 && addr <= 0x1FFF
        {
            if self.nCHRBanks == 0{
                *mapped_addr = addr as u32;
                return true;    
            }
        }
        return false;
    }

}