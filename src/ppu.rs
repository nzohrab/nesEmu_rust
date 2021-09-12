use crate::bits::*;
use crate::cartridge::*;
use arr_macro::*;

#[derive(Copy, Clone)]
pub struct Pixel(u8,u8,u8);

impl Pixel{
    pub fn to_u32(&self) -> u32{
        return ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | self.2 as u32;
    }
}

pub struct Sprite{
    width: i16,
    height: i16,
    pixels: Vec<Vec<Pixel>>
}

impl Sprite{
    pub fn SetPixel(&mut self, x: i16, y: i16, pixel: Pixel){
        //println!("{},{}", x,y);
        if x >= 0 && x < self.width && y >= 0 && y < self.height{
            self.pixels[y as usize][x as usize].0 = pixel.0;
            self.pixels[y as usize][x as usize].1 = pixel.1;
            self.pixels[y as usize][x as usize].2 = pixel.2;
        }
    }
}

pub fn mk_sprite(w: i16, h: i16) -> Sprite{
    let pixels: Vec<Vec<Pixel>> = (0..h).map(|_| (0..w).map(|_| Pixel(0,0,0)).collect()).collect();
    return Sprite {
        width: w,
        height: h,
        pixels: pixels,
    }
}

pub struct PPU{
    tblName: [[u8; 1024]; 2],
    tblPattern: [[u8; 4096]; 2],
    tblPalette: [u8; 32],
    palScreen: [Pixel; 0x40],
    sprScreen: Sprite,
    sprNameTable: [Sprite; 2],
    sprPatternTable: [Sprite; 2],
    pub scr_buf: [u32; 256*240],
    scanline: i16,
    cycle: i16,
    pub frame_complete: bool,
    pub cart: *mut Cartridge,
    pub stat_reg: BitMapper,
    pub mask_reg: BitMapper,
    pub ctrl_reg: BitMapper,
    
    addr_latch: u8,
    ppu_data_buffer: u8,
    vram_addr: BitMapper,
    tram_addr: BitMapper,
    fine_x: u8,
    pub nmi: bool,
    bg_next_tile_id: u8,
    bg_next_tile_attrib: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,
    bg_shifter_ptn_lo: u16,
    bg_shifter_ptn_hi: u16,
    bg_shifter_attrib_lo: u16,
    bg_shifter_attrib_hi: u16,
    
    
}

pub fn create_palScreen() -> [Pixel; 0x40]{
    let mut palScreen: [Pixel; 0x40] =  [Pixel(0,0,0); 0x40];

    palScreen[0x00] = Pixel(84, 84, 84);
	palScreen[0x01] = Pixel(0, 30, 116);
	palScreen[0x02] = Pixel(8, 16, 144);
	palScreen[0x03] = Pixel(48, 0, 136);
	palScreen[0x04] = Pixel(68, 0, 100);
	palScreen[0x05] = Pixel(92, 0, 48);
	palScreen[0x06] = Pixel(84, 4, 0);
	palScreen[0x07] = Pixel(60, 24, 0);
	palScreen[0x08] = Pixel(32, 42, 0);
	palScreen[0x09] = Pixel(8, 58, 0);
	palScreen[0x0A] = Pixel(0, 64, 0);
	palScreen[0x0B] = Pixel(0, 60, 0);
	palScreen[0x0C] = Pixel(0, 50, 60);
	palScreen[0x0D] = Pixel(0, 0, 0);
	palScreen[0x0E] = Pixel(0, 0, 0);
	palScreen[0x0F] = Pixel(0, 0, 0);

	palScreen[0x10] = Pixel(152, 150, 152);
	palScreen[0x11] = Pixel(8, 76, 196);
	palScreen[0x12] = Pixel(48, 50, 236);
	palScreen[0x13] = Pixel(92, 30, 228);
	palScreen[0x14] = Pixel(136, 20, 176);
	palScreen[0x15] = Pixel(160, 20, 100);
	palScreen[0x16] = Pixel(152, 34, 32);
	palScreen[0x17] = Pixel(120, 60, 0);
	palScreen[0x18] = Pixel(84, 90, 0);
	palScreen[0x19] = Pixel(40, 114, 0);
	palScreen[0x1A] = Pixel(8, 124, 0);
	palScreen[0x1B] = Pixel(0, 118, 40);
	palScreen[0x1C] = Pixel(0, 102, 120);
	palScreen[0x1D] = Pixel(0, 0, 0);
	palScreen[0x1E] = Pixel(0, 0, 0);
	palScreen[0x1F] = Pixel(0, 0, 0);

	palScreen[0x20] = Pixel(236, 238, 236);
	palScreen[0x21] = Pixel(76, 154, 236);
	palScreen[0x22] = Pixel(120, 124, 236);
	palScreen[0x23] = Pixel(176, 98, 236);
	palScreen[0x24] = Pixel(228, 84, 236);
	palScreen[0x25] = Pixel(236, 88, 180);
	palScreen[0x26] = Pixel(236, 106, 100);
	palScreen[0x27] = Pixel(212, 136, 32);
	palScreen[0x28] = Pixel(160, 170, 0);
	palScreen[0x29] = Pixel(116, 196, 0);
	palScreen[0x2A] = Pixel(76, 208, 32);
	palScreen[0x2B] = Pixel(56, 204, 108);
	palScreen[0x2C] = Pixel(56, 180, 204);
	palScreen[0x2D] = Pixel(60, 60, 60);
	palScreen[0x2E] = Pixel(0, 0, 0);
	palScreen[0x2F] = Pixel(0, 0, 0);

	palScreen[0x30] = Pixel(236, 238, 236);
	palScreen[0x31] = Pixel(168, 204, 236);
	palScreen[0x32] = Pixel(188, 188, 236);
	palScreen[0x33] = Pixel(212, 178, 236);
	palScreen[0x34] = Pixel(236, 174, 236);
	palScreen[0x35] = Pixel(236, 174, 212);
	palScreen[0x36] = Pixel(236, 180, 176);
	palScreen[0x37] = Pixel(228, 196, 144);
	palScreen[0x38] = Pixel(204, 210, 120);
	palScreen[0x39] = Pixel(180, 222, 120);
	palScreen[0x3A] = Pixel(168, 226, 144);
	palScreen[0x3B] = Pixel(152, 226, 180);
	palScreen[0x3C] = Pixel(160, 214, 228);
	palScreen[0x3D] = Pixel(160, 162, 160);
	palScreen[0x3E] = Pixel(0, 0, 0);
	palScreen[0x3F] = Pixel(0, 0, 0);
    return palScreen;
}

impl PPU{

    pub fn new() -> PPU{
        return PPU{
            tblName: [[0u8; 1024]; 2],
            tblPattern: [[0u8; 4096]; 2],
            tblPalette: [0u8; 32],
            palScreen: create_palScreen(),
            sprScreen: mk_sprite(256,240),
            sprNameTable: arr![mk_sprite(256, 240); 2],
            sprPatternTable: arr![mk_sprite(128, 128); 2],
            scr_buf: [0u32; 256*240],
            scanline: 0,
            cycle: 0,
            frame_complete: false,
            cart: std::ptr::null_mut(),

            stat_reg: crate::my_bits!{ //u8
                BitsId::Unused => 0..5,
                BitsId::SpriteOverflow => 5..6,
                BitsId::SpriteZeroHit => 6..7,
                BitsId::VerticalBlank => 7..8
            },

            mask_reg: crate::my_bits!{ //u8
                BitsId::Grayscale => 0..1,
                BitsId::RenderBackgroundLeft => 1..2,
                BitsId::RenderSpritesLeft => 2..3,
                BitsId::RenderBackground => 3..4,
                BitsId::RenderSprites => 4..5,
                BitsId::EnhanceRed => 5..6,
                BitsId::EnhanceGreen => 6..7,
                BitsId::EnhanceBlue => 7..8
            },

            //0b00010000
            ctrl_reg: crate::my_bits!{//u8

                BitsId::NametableX => 0..1,
                BitsId::NametableY => 1..2,
                BitsId::IncrementMode => 2..3,
                BitsId::PatternSprite => 3..4,
                BitsId::PatternBackground => 4..5,
                BitsId::SpriteSize => 5..6,
                BitsId::SlaveMode => 6..7,
                BitsId::EnableNmi => 7..8
            },

            addr_latch: 0x00,
            ppu_data_buffer: 0x00,
            vram_addr: crate::my_bits!{
                BitsId::CoarseX => 0..5,
                BitsId::CoarseY => 5..10,
                BitsId::NametableX => 10..11,
                BitsId::NametableY => 11..12,
                BitsId::FineY => 12..15,
                BitsId::Unused => 15..16
            },

            tram_addr: crate::my_bits!{
                BitsId::CoarseX => 0..5,
                BitsId::CoarseY => 5..10,
                BitsId::NametableX => 10..11,
                BitsId::NametableY => 11..12,
                BitsId::FineY => 12..15,
                BitsId::Unused => 15..16
            },
            fine_x: 0,
            nmi: false,
            bg_next_tile_id: 0,
            bg_next_tile_attrib: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            bg_shifter_ptn_lo: 0,
            bg_shifter_ptn_hi: 0,
            bg_shifter_attrib_lo: 0,
            bg_shifter_attrib_hi: 0,
        }    
    }

    pub fn get_colour_from_palette_ram(&self, palette: u8, pixel: u8) -> Pixel{
        return self.palScreen[(self.ppu_read(0x3F00 + ((palette as u16) << 2) + pixel as u16) & 0x3F) as usize];
    }

    pub fn get_pattern_table(&mut self, i: u8, palette: u8){
        for tY in 0..16{
            for tX in 0..16{
                let nOffset = tY*256+tX*16;
                for row in 0..8{
                    let mut tile_lsb = self.ppu_read((i as u16) * 0x1000 + nOffset + row + 0);
                    let mut tile_msb = self.ppu_read((i as u16) * 0x1000 + nOffset + row + 1);
                    for col in 0..8{
                        let pixel = (tile_lsb & 0x01) + (tile_msb & 0x01);
                        tile_lsb >>= 1;
                        tile_msb >>= 1;

                        self.sprPatternTable[i as usize].SetPixel(
                            (tX * 8 + (7 - col)) as i16,
                            (tY * 8 + row) as i16,
                            self.get_colour_from_palette_ram(palette, pixel)
                        )
                    }
                }
            }
        }
    }

    pub fn t(&mut self){
        // println!("testing");
        // self.stat_reg.bits = 0;
        // self.stat_reg.set(BitsId::VerticalBlank, 1);
        // println!("{}", self.stat_reg.bits);

        // self.stat_reg.bits = 0;
        // self.stat_reg.set(BitsId::Unused,1);
        // println!("{}", self.stat_reg.bits);

    }

    // pub fn get_fb(&mut self) -> [u32; 256*240]{ //Slow
    //     let mut buf: [u32; 256*240] = [0u32; 256*240];
    //     let mut i: usize = 0;
    //     for row in &self.sprScreen.pixels {
    //         for col in row{

    //             buf[i] = col.to_u32();
    //             i += 1;
    //         }
    //     }
    //     self.frame_complete = false;
    //     return buf;
    // }

    pub fn cpu_write(&mut self, address: u16, data: u8){



        match address{
            0x0000 => {
                self.ctrl_reg.bits = data as u16;
                self.tram_addr.set(BitsId::NametableX, self.ctrl_reg.get(BitsId::NametableX));
                self.tram_addr.set(BitsId::NametableY, self.ctrl_reg.get(BitsId::NametableY));
                //println!("{:#010b}", self.ctrl_reg.bits);
            }

            0x0001 => {
                self.mask_reg.bits = data as u16;
                //println!("{:#010b}", self.mask_reg.bits);
            }
            
            0x0002 => {}
            0x0003 => {}
            0x0004 => {}
            0x0005 => {
                if self.addr_latch == 0 {
                    self.fine_x = data & 0x07;
                    self.tram_addr.set(BitsId::CoarseX, data as u16 >> 3);
                    self.addr_latch = 1
                } else {
                    self.tram_addr.set(BitsId::FineY, data as u16 & 0x07);
                    self.tram_addr.set(BitsId::CoarseY, data as u16 >> 3);
                    self.addr_latch = 0;
                }

            }
            0x0006 => {
                if self.addr_latch == 0 {
                    self.tram_addr.bits = (((data as u16 & 0x3F) << 8) | ((self.tram_addr.bits as u16) & 0x00FF));
                    self.addr_latch = 1
                } else {
                    self.tram_addr.bits = ((self.tram_addr.bits & 0xFF00) as u16 | data as u16);
                    self.vram_addr.bits = self.tram_addr.bits;
                    self.addr_latch = 0;
                }
            }
            0x0007 => {
                self.ppu_write(self.vram_addr.bits as u16, data); 
                self.vram_addr.bits = (self.vram_addr.bits + if self.ctrl_reg.get(BitsId::IncrementMode) == 1 {32} else {1});
            }
            _ => {}

        }
    }    

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let mut data: u8 = 0x00;
        match address {
            0x0000 => {}
            0x0001 => {}
            0x0002 => {
                //self.stat_reg.set_vertical_blank(1);
                data = (self.stat_reg.bits as u8 & 0xE0) | (self.ppu_data_buffer & 0x1F);
                self.stat_reg.set(BitsId::VerticalBlank,0);
                self.addr_latch = 0;
            }
            0x0003 => {}
            0x0004 => {}
            0x0005 => {}
            0x0006 => {}
            0x0007 => {
                data = self.ppu_data_buffer;
                self.ppu_data_buffer = self.ppu_read(self.vram_addr.bits as u16);
                
                if self.vram_addr.bits >= 0x3F00{
                     data = self.ppu_data_buffer;
                }
                self.vram_addr.bits = (self.vram_addr.bits + if self.ctrl_reg.get(BitsId::IncrementMode) == 1 {32} else {1});
            }
            _ => {}

        } 
        return data;
    }

    pub fn ppu_read(&self, address: u16) -> u8 {
        let mut data: u8 = 0x00;
        let mut m_address = address & 0x3FFF;


        unsafe{
            if (*self.cart).ppu_read(m_address, &mut data) {

            } else if m_address >= 0x0000 && m_address <= 0x1FFF{
                data = self.tblPattern[((m_address & 0x1000) >> 12) as usize][(m_address & 0x0FFF) as usize];
            }else if m_address >= 0x2000 && m_address <= 0x3EFF{
                m_address &= 0x0FFF;
                if (*self.cart).mirror == Mirror::Vertical{
                    if m_address >= 0x0000 && m_address <= 0x03FF {data = self.tblName[0][(m_address &0x03FF) as usize];}
                    if m_address >= 0x0400 && m_address <= 0x07FF {data = self.tblName[1][(m_address &0x03FF) as usize];}
                    if m_address >= 0x0800 && m_address <= 0x0BFF {data = self.tblName[0][(m_address &0x03FF) as usize];}
                    if m_address >= 0x0C00 && m_address <= 0x0FFF {data = self.tblName[1][(m_address &0x03FF) as usize];}
                } else if (*self.cart).mirror == Mirror::Horizontal{
                    if m_address >= 0x0000 && m_address <= 0x03FF {data = self.tblName[0][(m_address &0x03FF) as usize];}
                    if m_address >= 0x0400 && m_address <= 0x07FF {data = self.tblName[0][(m_address &0x03FF) as usize];}
                    if m_address >= 0x0800 && m_address <= 0x0BFF {data = self.tblName[1][(m_address &0x03FF) as usize];}
                    if m_address >= 0x0C00 && m_address <= 0x0FFF {data = self.tblName[1][(m_address &0x03FF) as usize];}
                }
            }else if m_address >= 0x3F00 && m_address <= 0x3FFF{
                m_address &= 0x001F;
                if m_address == 0x0010 {m_address = 0x0000}
                if m_address == 0x0014 {m_address = 0x0004}
                if m_address == 0x0018 {m_address = 0x0008}
                if m_address == 0x001C {m_address = 0x000C}
                data = self.tblPalette[m_address as usize] & (if self.mask_reg.get(BitsId::Grayscale) != 0 {0x30} else {0x3F})
            }
        }
        return data;
    }

    pub fn ppu_write(&mut self, address: u16, data: u8){

        let mut m_address = address & 0x3FFF; 
        unsafe{
            if (*self.cart).ppu_write(m_address, data) {
            
            } else if m_address >= 0x0000 && m_address <= 0x1FFF{
                self.tblPattern[((m_address & 0x1000) >> 12) as usize][(m_address & 0x0FFF) as usize] = data;

            }else if m_address >= 0x2000 && m_address <= 0x3EFF{
                m_address &= 0x0FFF;
                if (*self.cart).mirror == Mirror::Vertical{
                    if m_address >= 0x0000 && m_address <= 0x03FF {self.tblName[0][(m_address &0x03FF) as usize] = data;}
                    if m_address >= 0x0400 && m_address <= 0x07FF {self.tblName[1][(m_address &0x03FF) as usize] = data;}
                    if m_address >= 0x0800 && m_address <= 0x0BFF {self.tblName[0][(m_address &0x03FF) as usize] = data;}
                    if m_address >= 0x0C00 && m_address <= 0x0FFF {self.tblName[1][(m_address &0x03FF) as usize] = data;}
                } else if (*self.cart).mirror == Mirror::Horizontal{
                    if m_address >= 0x0000 && m_address <= 0x03FF {self.tblName[0][(m_address &0x03FF) as usize] = data;}
                    if m_address >= 0x0400 && m_address <= 0x07FF {self.tblName[0][(m_address &0x03FF) as usize] = data;}
                    if m_address >= 0x0800 && m_address <= 0x0BFF {self.tblName[1][(m_address &0x03FF) as usize] = data;}
                    if m_address >= 0x0C00 && m_address <= 0x0FFF {self.tblName[1][(m_address &0x03FF) as usize] = data;}
                }
            }else if m_address >= 0x3F00 && m_address <= 0x3FFF{
                m_address &= 0x001F;
                if m_address == 0x0010 {m_address = 0x0000}
                if m_address == 0x0014 {m_address = 0x0004}
                if m_address == 0x0018 {m_address = 0x0008}
                if m_address == 0x001C {m_address = 0x000C}
                self.tblPalette[m_address as usize] = data;
            }
        }
    }

    fn inc_scroll_x(&mut self) {
        if self.mask_reg.get(BitsId::RenderBackground) != 0 || self.mask_reg.get(BitsId::RenderSprites) != 0{
            if self.vram_addr.get(BitsId::CoarseX) == 31{
                self.vram_addr.set(BitsId::CoarseX, 0);

                self.vram_addr.set(BitsId::NametableX, !self.vram_addr.get(BitsId::NametableX));
            } else{
                self.vram_addr.set(BitsId::CoarseX, self.vram_addr.get(BitsId::CoarseX) + 1);
            }
        } 
    }

    fn inc_scroll_y(&mut self) {
        if self.mask_reg.get(BitsId::RenderBackground) != 0 || self.mask_reg.get(BitsId::RenderSprites) != 0{
            if self.vram_addr.get(BitsId::FineY) < 7{
                self.vram_addr.set(BitsId::FineY, self.vram_addr.get(BitsId::FineY) + 1);
            } else{
                self.vram_addr.set(BitsId::FineY, 0);

                if self.vram_addr.get(BitsId::CoarseY) == 29{
                    self.vram_addr.set(BitsId::CoarseY, 0);
    
                    self.vram_addr.set(BitsId::NametableY, !self.vram_addr.get(BitsId::NametableY));
                } else if self.vram_addr.get(BitsId::CoarseY) == 31{
                    self.vram_addr.set(BitsId::CoarseY, 0);
                }else{
                    self.vram_addr.set(BitsId::CoarseY, self.vram_addr.get(BitsId::CoarseY) + 1);
                }
            }
        } 
    }

    fn transfer_addr_x(&mut self){
        if self.mask_reg.get(BitsId::RenderBackground) != 0 || self.mask_reg.get(BitsId::RenderSprites) != 0{
            self.vram_addr.set(BitsId::NametableX, self.tram_addr.get(BitsId::NametableX));
            self.vram_addr.set(BitsId::CoarseX, self.tram_addr.get(BitsId::CoarseX));
        }
    }

    fn transfer_addr_y(&mut self){
        if self.mask_reg.get(BitsId::RenderBackground) != 0 || self.mask_reg.get(BitsId::RenderSprites) != 0{
            self.vram_addr.set(BitsId::FineY, self.tram_addr.get(BitsId::FineY));

            self.vram_addr.set(BitsId::NametableY, self.tram_addr.get(BitsId::NametableY));
            self.vram_addr.set(BitsId::CoarseY, self.tram_addr.get(BitsId::CoarseY));
        }
    }

    fn load_bg_shifters(&mut self) {
        self.bg_shifter_ptn_lo = (self.bg_shifter_ptn_lo & 0xFF00) | self.bg_next_tile_lsb as u16;
        self.bg_shifter_ptn_hi = (self.bg_shifter_ptn_hi & 0xFF00) | self.bg_next_tile_msb as u16;
        self.bg_shifter_attrib_lo = (self.bg_shifter_attrib_lo & 0xFF00) | (if (self.bg_next_tile_attrib & 0b01) != 0 {0xFF} else {0x00});
        self.bg_shifter_attrib_hi = (self.bg_shifter_attrib_hi & 0xFF00) | (if (self.bg_next_tile_attrib & 0b10) != 0 {0xFF} else {0x00});
    }

    fn update_shifters(&mut self){
        if self.mask_reg.get(BitsId::RenderBackground) != 0{
            self.bg_shifter_ptn_lo <<= 1;
            self.bg_shifter_ptn_hi <<= 1;
            self.bg_shifter_attrib_lo <<= 1;
            self.bg_shifter_attrib_hi <<= 1;
        }
    }


    pub fn clock(&mut self){
        //self.mask_reg.set(BitsId::RenderBackground, 1);
        //println!("{}",self.mask_reg.get(BitsId::RenderBackground));


        if self.scanline >= -1 && self.scanline < 240{
            if self.scanline == -1 && self.cycle == 1{
                self.stat_reg.set(BitsId::VerticalBlank,0);
            }
            if self.cycle >= 2 && self.cycle < 258 || self.cycle >= 321 && self.cycle < 338{
                self.update_shifters();
                match (self.cycle-1) % 8{
                    0 => {
                        self.load_bg_shifters();
                        self.bg_next_tile_id = self.ppu_read(0x2000 | (self.vram_addr.bits as u16 & 0x0FFF));
                    }
                    2 => {
                        self.bg_next_tile_attrib = self.ppu_read((0x23C0|   (self.vram_addr.get(BitsId::NametableY) << 11)
                                                                        |   (self.vram_addr.get(BitsId::NametableX) << 10)
                                                                        |   ((self.vram_addr.get(BitsId::CoarseY) >> 2) << 3)
                                                                        |   (self.vram_addr.get(BitsId::CoarseX) >> 2)) as u16); //Caution
                        
                        if (self.vram_addr.get(BitsId::CoarseY) & 0x02) != 0{
                            self.bg_next_tile_attrib >>= 4;
                        }
                        if (self.vram_addr.get(BitsId::CoarseX) & 0x02) != 0{
                            self.bg_next_tile_attrib >>= 2;
                        }
                        self.bg_next_tile_attrib &= 0x03;
                    }

                    4 => {
                        self.bg_next_tile_lsb = self.ppu_read(((self.ctrl_reg.get(BitsId::PatternBackground) << 12)
                            +   ((self.bg_next_tile_id as u16) << 4) 
                            +   (self.vram_addr.get(BitsId::FineY) + 0)) as u16)
                    }
                    6 => {
                        self.bg_next_tile_msb = self.ppu_read(((self.ctrl_reg.get(BitsId::PatternBackground) << 12)
                        +   ((self.bg_next_tile_id as u16) << 4) 
                        +   (self.vram_addr.get(BitsId::FineY) + 8)) as u16)

                    }
                    7 => {
                        self.inc_scroll_x();
                    }
                    _ => {}
                }
            }
            if self.cycle == 256{
                self.inc_scroll_y();
            }

            if self.cycle == 257{
                self.load_bg_shifters();
                self.transfer_addr_x();
            }

            if self.cycle == 338 || self.cycle == 340{
                self.bg_next_tile_id = self.ppu_read(0x2000 | (self.vram_addr.bits as u16 & 0x0FFF));
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305{
                self.transfer_addr_y();
            }
        }

        if self.scanline == 240{
            // Do nothing
        }

        if self.scanline >= 241 && self.scanline < 261 {
            if self.scanline == 241 && self.cycle == 1{
                self.stat_reg.set(BitsId::VerticalBlank,1);
                if self.ctrl_reg.get(BitsId::EnableNmi) == 1{
                    self.nmi = true;
                }
            }
        }

        let mut bg_pixel: u8 = 0x00;
        let mut bg_pal: u8 = 0x00;
        if self.mask_reg.get(BitsId::RenderBackground) != 0{
            //println!("rendering");
            let bit_mux = 0x8000 >> self.fine_x;

            let p0_pixel: u8 = if (self.bg_shifter_ptn_lo & bit_mux) > 0 {1} else {0};
            let p1_pixel: u8 = if (self.bg_shifter_ptn_hi & bit_mux) > 0 {1} else {0};
            bg_pixel = (p1_pixel << 1) | p0_pixel;

            let bg_pal0: u8 = if (self.bg_shifter_attrib_lo & bit_mux) > 0 {1} else {0};
            let bg_pal1: u8 = if (self.bg_shifter_attrib_hi & bit_mux) > 0 {1} else {0};
            bg_pal = (bg_pal1 << 1) | bg_pal0;
        }

        let px = self.get_colour_from_palette_ram(bg_pal, bg_pixel);
        //println!("{}, {},   ({},{},{})", bg_pixel, bg_pal, px.0,px.1,px.2);

        //self.sprScreen.SetPixel((self.cycle - 1) as i16, (self.scanline) as i16, px);
        let x: i32 = (self.cycle - 1) as i32;
        let y: i32 = self.scanline as i32;
        if x >= 0 && x < 256 && y >= 0 && y < 240{
            //println!("{},{},{}", (y * 256 + x),x,y);
            self.scr_buf[(y * 256 + x) as usize] = px.to_u32();
        }

        self.cycle += 1;
        if self.cycle >= 341{
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261{
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
    }
}