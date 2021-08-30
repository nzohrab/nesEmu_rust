use crate::mappers::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem;
use std::slice;
use std::io::SeekFrom;
use std::vec;

pub struct CartHeader{
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 5],
}

#[derive(PartialEq)]
pub enum Mirror{
    Vertical,
    Horizontal,
    OnScreenLO,
    OnScreenHI,
}

pub struct Cartridge{
    cart_header: CartHeader,
    nMapperID: u8,
    nPRGBanks: u8,
    nCHRBanks: u8,
    vPRGMemory: Vec<u8>,
    vCHRMemory: Vec<u8>,
    pub mapper: Mapper000,
    pub mirror: Mirror,
    imageIsValid: bool,
}

pub fn read_n<R>(reader: &mut BufReader<R>, bytes: &mut Vec<u8>) -> std::io::Result<()>
where
    R: Read,
{
    let mut buf = vec![0u8; (&bytes).len()];
    reader.read_exact(&mut buf)?;
    //assert_eq!(bytes.len(), buf.len());
    let old = mem::replace(bytes, buf);
    

    //bytes.extend(buf);
    Ok(())
}


impl Cartridge{

    pub fn new(filename: String) -> std::io::Result<Cartridge>{
        let f = File::open(filename)?;
        let mut reader = BufReader::new(f);


        let header_size = mem::size_of::<CartHeader>();

        let cart_header: CartHeader = {
            let mut h = [0u8; 16];

            reader.read_exact(&mut h[..])?;

            unsafe { mem::transmute(h) }
        };

        if (cart_header.mapper1 & 0x04 as u8) != 0 {
            reader.seek(SeekFrom::Current(512));
        }

        let mapperID = ((cart_header.mapper2 >> 4) << 4) | (cart_header.mapper1 >> 4);
        let mirror = if (cart_header.mapper1 & 0x01) != 0 {Mirror::Vertical} else {Mirror::Horizontal};
        
        let mut vPRGMemory: Vec<u8> = Vec::new();
        let mut vCHRMemory: Vec<u8> = Vec::new();


        let fileType = 1;

        let nPRGBanks = cart_header.prg_rom_chunks;
        vPRGMemory.resize(16384 * nPRGBanks as usize, 0 as u8);

        read_n(&mut reader, &mut vPRGMemory);

        let nCHRBanks = cart_header.chr_rom_chunks;
        vCHRMemory.resize(8192 * nCHRBanks as usize, 0 as u8);

        read_n(&mut reader, &mut vCHRMemory);

        let mapper: Mapper000 = Mapper000::new(nPRGBanks, nCHRBanks);
        let cart = Cartridge{
            cart_header: cart_header,
            nMapperID: mapperID,
            nPRGBanks: nPRGBanks,
            nCHRBanks: nCHRBanks,
            vPRGMemory: vPRGMemory,
            vCHRMemory: vCHRMemory,
            mapper: mapper,
            mirror: mirror,
            imageIsValid: true,        
        };
        Ok(cart)
    }



    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool{
        let mut mapped_addr: u32 = 0;
        if self.mapper.cpu_map_write(addr, &mut mapped_addr){
            self.vPRGMemory[mapped_addr as usize] = data;
            return true;
        }
        return false;
    }

    pub fn cpu_read(&self, addr: u16, data: &mut u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if self.mapper.cpu_map_read(addr, &mut mapped_addr){
            *data = self.vPRGMemory[mapped_addr as usize];
            return true;
        }
        return false;
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool{
        let mut mapped_addr: u32 = 0;
        if self.mapper.ppu_map_write(addr, &mut mapped_addr){
            self.vCHRMemory[mapped_addr as usize] = data;
            return true;
        }
        return false;
    }

    pub fn ppu_read(&self, addr: u16, data: &mut u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if self.mapper.ppu_map_read(addr, &mut mapped_addr){
            *data = self.vCHRMemory[mapped_addr as usize];
            return true;
        }
        return false;
    }
}