use crate::chip::*;
use crate::ppu::*;
use crate::cartridge::*;
use std::ptr;

pub struct Bus {
    cpu_ram: [u8; 0x0800],
    clock_counter: u128,
    cart: *mut Cartridge,
    pub ppu: *mut PPU,
    cpu: *mut Chip,
    controller_state: [u8; 2],
    pub controllers: [u8; 2],
}

impl Bus{
    pub fn new(cart: &mut Cartridge, ppu: &mut PPU, cpu: &mut Chip) -> Self {
        Self {
            cpu_ram: [0; 0x0800],
            clock_counter: 0,
            cart: cart,
            ppu: ppu,
            cpu: cpu,
            controller_state: [0; 2],
            controllers: [0; 2],
        }
    }

    pub fn set_cpu_ptr(&mut self, cpu_ptr: *mut Chip){
        self.cpu = cpu_ptr;
    }

    pub fn cpu_write(&mut self, address: u16, data: u8){
        unsafe{
            if (*self.cart).cpu_write(address, data){

            }else if address >= 0 && address <= 0x1FFF{
                self.cpu_ram[(address & 0x07FF) as usize] = data;
            } else if address >= 0x2000 && address <= 0x3FFF{
                (*self.ppu).cpu_write(address & 0x0007,  data);
            }else if address >= 0x4016 && address <= 0x4017{
                self.controller_state[(address & 0x0001) as usize] = self.controllers[(address & 0x0001) as usize];
            }
        }
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let mut data: u8 = 0x00;
        unsafe{

            if (*self.cart).cpu_read(address, &mut data){

            } else if address >= 0x0000 && address <= 0x1FFF {
                data = self.cpu_ram[(address & 0x07FF) as usize];
            } else if address >= 0x2000 && address <= 0x3FFF{
                    data = (*self.ppu).cpu_read(address & 0x0007);
                
            }else if address >= 0x4016 && address <= 0x4017{
                data = if (self.controller_state[(address & 0x0001) as usize] & 0x80) > 0 {1} else {0};
                self.controller_state[(address & 0x0001) as usize] <<= 1;
            }
        }  
        return data;
    }

    pub fn reset(&self){
        unsafe{
            (*self.cpu).reset();
        }
    }

    pub fn clock(&mut self){
        unsafe{
            (*self.ppu).clock();
            if(self.clock_counter % 3 == 0){
                (*self.cpu).clock();
            }

            if (*self.ppu).nmi{
                (*self.ppu).nmi = false;
                (*self.cpu).nmi();
            }

            self.clock_counter += 1;
        }
    }
}    
