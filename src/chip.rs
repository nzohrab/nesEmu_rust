use crate::bus::*;
use crate::instructions::*;

pub struct Chip{
    pub instrucs: [Instruction; 256],
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub stkp: u8,
    pub pc: u16,
    pub flags: u8,
    pub fetched: u8,
    pub temp: u16,
    pub operation: u8,
    pub cycles_remaining: u8,
    pub addr_rel: u16,
    pub addr_abs: u16,
    pub data_bus: *mut Bus,
    pub clock_count: u64,
}

pub enum Flag {
    C = (1 << 0),
    Z = (1 << 1),
    I = (1 << 2),
    D = (1 << 3),
    B = (1 << 4),
    U = (1 << 5),
    V = (1 << 6),
    N = (1 << 7)
}


impl Chip {
    pub fn new() -> Chip{
        return Chip{instrucs: get_instruction_defs(),
                    a: 0,
                    x: 0,
                    y: 0,
                    stkp: 0,
                    pc: 0,
                    flags: 0,
                    fetched: 0,
                    temp: 0,
                    operation: 0,
                    cycles_remaining: 0,
                    addr_rel: 0,
                    addr_abs: 0,
                    data_bus: std::ptr::null_mut(),
                    clock_count: 0,
                };
    }

    pub fn register_string(&self) -> String{
        let gen = format!("        a={:#04x}            x={:#04x}            y={:#04x}", self.a, self.x, self.y);
  let addresses = format!("     stkp={:#04x}           pc={:#04x}     addr_abs={:#04x}    addr_rel={:#04x}", self.stkp, self.pc, self.addr_abs, self.addr_rel);
   let op_state = format!("operation={:#04x}      fetched={:#04x}         temp={:#04x}  cyc_remain={}", self.operation, self.fetched, self.temp, self.cycles_remaining);
        let status = format!("C={} Z={} I={} D={} B={} U={} V={} N={}", self.get_flag(Flag::C),self.get_flag(Flag::Z),self.get_flag(Flag::I),self.get_flag(Flag::D),self.get_flag(Flag::B),self.get_flag(Flag::U),self.get_flag(Flag::V),self.get_flag(Flag::N));        

        return format!("{}\n{}\n{}\n{}\n", gen,addresses,op_state,status);
    }

    pub fn write(&mut self, address: u16, data: u8){
        
        unsafe{
            (*self.data_bus).cpu_write(address, data);
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        unsafe{
            return (*self.data_bus).cpu_read(address);
        }
    }

    pub fn reset(&mut self){
        self.addr_abs = 0xFFFC;
        let lo: u16 = self.read(self.addr_abs + 0) as u16;
        let hi: u16 = self.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.flags = 0x00; 
        self.set_flag(Flag::U, true);

        self.addr_abs = 0x0000;
        self.addr_rel = 0x0000;
        self.fetched = 0x00;

        self.cycles_remaining = 8;
    }

    pub fn fetch(&mut self) -> u8 {
        if self.instrucs[self.operation as usize].addr_mode != AddrMode::IMP{
            self.fetched = self.read(self.addr_abs);
            //println!("fetched: {}", self.fetched);

        }
        return self.fetched;
    }    

    pub fn call_addr_mode(&mut self,addr_mode: AddrMode) -> u8 {
        match addr_mode {

            AddrMode::IMP => return self.IMP(),
            AddrMode::IMM => return self.IMM(),
            AddrMode::ZP0 => return self.ZP0(),
            AddrMode::ZPX => return self.ZPX(),
            AddrMode::ZPY => return self.ZPY(),
            AddrMode::REL => return self.REL(),
            AddrMode::ABX => return self.ABX(),
            AddrMode::ABS => return self.ABS(),
            AddrMode::ABY => return self.ABY(),
            AddrMode::IND => return self.IND(),
            AddrMode::IZX => return self.IZX(),
            AddrMode::IZY => return self.IZY(),

            _ => println!("Could not call operation / address mode"),
        }
        return 0;

    }

    pub fn call_op(&mut self,fn_name: Operation) -> u8{
        match fn_name {
            Operation::BRK => return self.BRK(),
            Operation::ORA => return self.ORA(),
            Operation::XXX => return self.XXX(),
            Operation::NOP => return self.NOP(),
            Operation::ASL => return self.ASL(),
            Operation::PHP => return self.PHP(),
            Operation::BPL => return self.BPL(),
            Operation::CLC => return self.CLC(),
            Operation::JSR => return self.JSR(),
            Operation::AND => return self.AND(),
            Operation::BIT => return self.BIT(),
            Operation::ROL => return self.ROL(),
            Operation::PLP => return self.PLP(),
            Operation::BMI => return self.BMI(),
            Operation::SEC => return self.SEC(),
            Operation::RTI => return self.RTI(),
            Operation::EOR => return self.EOR(),
            Operation::LSR => return self.LSR(),
            Operation::PHA => return self.PHA(),
            Operation::JMP => return self.JMP(),
            Operation::BVC => return self.BVC(),
            Operation::CLI => return self.CLI(),
            Operation::RTS => return self.RTS(),
            Operation::ADC => return self.ADC(),
            Operation::ROR => return self.ROR(),
            Operation::PLA => return self.PLA(),
            Operation::BVS => return self.BVS(),
            Operation::SEI => return self.SEI(),
            Operation::STA => return self.STA(),
            Operation::STY => return self.STY(),
            Operation::STX => return self.STX(),
            Operation::DEY => return self.DEY(),
            Operation::TXA => return self.TXA(),
            Operation::BCC => return self.BCC(),
            Operation::TYA => return self.TYA(),
            Operation::TXS => return self.TXS(),
            Operation::LDY => return self.LDY(),
            Operation::LDA => return self.LDA(),
            Operation::LDX => return self.LDX(),
            Operation::TAY => return self.TAY(),
            Operation::TAX => return self.TAX(),
            Operation::BCS => return self.BCS(),
            Operation::CLV => return self.CLV(),
            Operation::TSX => return self.TSX(),
            Operation::CPY => return self.CPY(),
            Operation::CMP => return self.CMP(),
            Operation::DEC => return self.DEC(),
            Operation::INY => return self.INY(),
            Operation::DEX => return self.DEX(),
            Operation::BNE => return self.BNE(),
            Operation::CLD => return self.CLD(),
            Operation::CPX => return self.CPX(),
            Operation::SBC => return self.SBC(),
            Operation::INC => return self.INC(),
            Operation::INX => return self.INX(),
            Operation::BEQ => return self.BEQ(),
            Operation::SED => return self.SED(),
            
            _ => println!("Could not call operation / address mode"),
        }
        return 0;
    }

    //Address modes
    pub fn IMP(&mut self) -> u8{
        self.fetched = self.a;
        return 0 as u8;
    }
    pub fn IMM(&mut self) -> u8{
        self.addr_abs = self.pc;
        self.pc += 1;
        return 0 as u8;
    }
    pub fn ZP0(&mut self) -> u8{
        self.addr_abs = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0 as u8;
    }
    pub fn ZPX(&mut self) -> u8{
        self.addr_abs = (self.read(self.pc) + self.x) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0 as u8;
    }
    pub fn ZPY(&mut self) -> u8{
        self.addr_abs = (self.read(self.pc)+ self.y) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0 as u8;
    }
    pub fn REL(&mut self) -> u8{
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;
        if ((1 << 7) as u16 & self.addr_rel) != 0{
            self.addr_rel |= 0xFF00;
        }
        return 0 as u8;
    }
    pub fn ABS(&mut self) -> u8{
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        return 0 as u8;
    }

    pub fn ABX(&mut self) -> u8{
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;
        if (self.addr_abs & 0xFF00) != (hi << 8){
            return 1 as u8;
        } else {
            return 0 as u8;
        }
    }
    pub fn ABY(&mut self) -> u8{
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;
        if (self.addr_abs & 0xFF00) != (hi << 8){
            return 1 as u8;
        } else {
            return 0 as u8;
        }
    }
    pub fn IND(&mut self) -> u8{

        let ptr_lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let ptr_hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;
        if ptr_lo == 0x00FF{ //Deliberate bug
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr + 0) as u16;
        } else{ //Normal operation
            self.addr_abs = (self.read(ptr + 1) as u16) << 8 | self.read(ptr + 0) as u16;
        }

        return 0 as u8;
    }
    pub fn IZX(&mut self) -> u8{

        let t: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.read((t + (self.x as u16)) & 0x00FF) as u16;
        let hi: u16 = self.read((t + 1 + (self.x as u16)) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        return 0 as u8;
    }
    pub fn IZY(&mut self) -> u8{

        let t: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.read(t & 0x00FF) as u16;
        let hi: u16 = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;
        if (self.addr_abs & 0xFF00) != (hi << 8){
            return 1 as u8;
        } else {
            return 0 as u8;
        }
    }









    pub fn XXX(&self) -> u8{
        return 0 as u8;
    }

    pub fn BRK(&mut self) -> u8{
        self.pc -= 1;

        self.set_flag(Flag::I, true);
        self.write(0x0100 + self.stkp as u16, (self.pc >> 8) as u8);
        self.stkp -= 1;
        self.write(0x0100 + self.stkp as u16, self.pc as u8);

        self.stkp -= 1;

        self.set_flag(Flag::B, true);
        self.write(0x0100 + self.stkp as u16, self.flags);

        self.stkp -= 1;
        self.set_flag(Flag::B, false);

        self.pc = self.read(0xFFFE) as u16 | ((self.read(0xFFFF) as u16) << 8);
        return 0 as u8;
    }

    pub fn ORA(&mut self) -> u8{ 
        self.fetch();
        self.a = self.a | self.fetched;
        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, ((1 << 7) as u8 & self.a) != 0);
        return 1 as u8;
    }

    pub fn NOP(&self) -> u8{ 
        match self.operation {
            0x1C => return 1 as u8,
            0x3C => return 1 as u8,
            0x5C => return 1 as u8,
            0x7C => return 1 as u8,
            0xDC => return 1 as u8,
            0xFC => return 1 as u8,
            _ => return 0 as u8,
        }
    }
    pub fn ASL(&mut self) -> u8{

        self.fetch();
        self.temp = (self.fetched as u16) << 1;
        self.set_flag(Flag::C, (self.temp & 0xFF00) > 0);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, ((1 << 7) as u16 & self.temp) != 0); //Should be ok??
        if self.instrucs[self.operation as usize].addr_mode == AddrMode::IMP{
            self.a = self.temp as u8;
        } else {
            self.write(self.addr_abs, self.temp as u8);
        }
        return 0 as u8;
    }

    pub fn PHP(&mut self) -> u8{ 
        self.write(0x0100 + self.stkp as u16, self.flags | Flag::B as u8 | Flag::U as u8);
        self.set_flag(Flag::B, false);
        self.set_flag(Flag::U, false);
        self.stkp = self.stkp - 1;
        return 0 as u8;
    }
    pub fn CLC(&mut self) -> u8{
        self.set_flag(Flag::C, false);
        return 0 as u8;
    }

    pub fn JSR(&mut self) -> u8{ 
        self.pc = self.pc - 1;

        self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & (0x00FF as u16)) as u8); // This is not safe
        self.stkp = self.stkp - 1;
        self.write(0x0100 + self.stkp as u16, (self.pc & (0x00FF as u16)) as u8); // This is not safe
        self.stkp = self.stkp - 1;

        self.pc = self.addr_abs;
        return 0 as u8;
    }

    pub fn AND(&mut self) -> u8{ 
        self.fetch();
        self.a = self.a & self.fetched;
        self.set_flag(Flag::Z, self.a==0);
        self.set_flag(Flag::N, ((1 << 7) as u8 & self.a) != 0);
        return 0 as u8;
    }

    pub fn BIT(&mut self) -> u8{
        self.fetch();
        self.temp = self.a as u16 & self.fetched as u16;
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);

        self.set_flag(Flag::N, self.fetched & (1 << 7) != 0); //Should be ok??
        self.set_flag(Flag::V, self.fetched & (1 << 6) != 0); //Should be ok??
        return 0 as u8;
    }

    pub fn ROL(&mut self) -> u8{ 
        self.fetch();
        self.temp = (((self.fetched as u16) << 1) | (if self.get_flag(Flag::C) {1} else {0})) as u16;
        self.set_flag(Flag::C, (self.temp as u16 & 0xFF00) != 0);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, (self.temp & 0x0080) != 0); //Should be ok??
        if self.instrucs[self.operation as usize].addr_mode == AddrMode::IMP{
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (self.temp & 0x00FF) as u8);
        }
        return 0 as u8;
    }

    pub fn PLP(&mut self) -> u8{ 
        self.stkp = self.stkp + 1;
        self.flags = self.read(0x0100 + self.stkp as u16);
        self.set_flag(Flag::U, true);
        return 0 as u8;
    }
    pub fn SEC(&mut self) -> u8{ 
        self.set_flag(Flag::C, true);
        return 0 as u8;
    }
    pub fn RTI(&mut self) -> u8{
        self.stkp = self.stkp + 1;
        self.flags = self.read(0x0100 + self.stkp as u16);
        self.set_flag(Flag::B, false); //Pretty sure this is correct
        self.set_flag(Flag::U, false);
        self.stkp = self.stkp + 1;
        self.pc = self.read(0x0100 + self.stkp as u16) as u16;
        self.stkp = self.stkp + 1;
        self.pc |= ((self.read(0x0100 + self.stkp as u16) as u16) << 8);
        return 0 as u8;
    }
    pub fn EOR(&mut self) -> u8{
        self.fetch();
        self.a = self.a ^ self.fetched;
        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, ((1 << 7) as u8 & self.a) != 0);
        return 0 as u8;
    }

    pub fn LSR(&mut self) -> u8{ 
        self.fetch();
        self.set_flag(Flag::C, (self.fetched & 0x0001) != 0); //Unsure about int sizes here
        self.temp = (self.fetched >> 1) as u16;
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, (self.temp & 0x0080) != 0); //Should be ok??
        if self.instrucs[self.operation as usize].addr_mode == AddrMode::IMP{
            self.a = self.temp as u8; //Cast to u8?
        } else {
            self.write(self.addr_abs, self.temp as u8);
        }
        return 0 as u8;
    
    }
    pub fn PHA(&mut self) -> u8{ 
        self.write(0x0100 + self.stkp as u16, self.a);
        self.stkp = self.stkp - 1;
        return 0 as u8;
    }
    pub fn JMP(&mut self) -> u8{ 
        self.pc = self.addr_abs;
        return 0 as u8;
    }
    pub fn BVC(&mut self) -> u8{
        if !self.get_flag(Flag::V){
            self.cycles_remaining += 1;
            self.addr_abs = self.pc + (self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles_remaining += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0 as u8;
    }
    pub fn CLI(&mut self) -> u8{
        self.set_flag(Flag::I, false);
        return 0 as u8;
    }

    pub fn RTS(&mut self) -> u8{
        self.stkp = self.stkp + 1;
        self.pc = self.read(0x0100 + self.stkp as u16) as u16;
        self.stkp = self.stkp + 1;
        self.pc |= ((self.read(0x0100 + self.stkp as u16) as u16) << 8);
        self.pc += 1;
        return 0 as u8;
    }

    pub fn ADC(&mut self) -> u8{
        self.fetch();
        self.temp = self.a as u16 + self.fetched as u16 + (if self.get_flag(Flag::C) {1} else {0});

        self.set_flag(Flag::C, self.temp > 255);
        self.set_flag(Flag::Z, (self.temp as u16 & 0x00FF) == 0x0000);
        //self.set_flag(Flag::V, !(((self.a as u16) ^ (self.fetched as u16)) & ((self.a as u16) ^ (self.temp as u16)) & ((1 << 7) as u16)) != 0); //Be weary
        self.set_flag(Flag::V, (!((self.a as u16) ^ (self.fetched as u16)) & ((self.a as u16) ^ (self.temp as u16)) & ((1 << 7) as u16)) != 0); //Be weary
        self.set_flag(Flag::N, (self.temp & 0x0080) != 0); //Should be ok??
        self.a = self.temp as u8;
        return 1 as u8;
    }
    pub fn INC(&mut self) -> u8{
        self.fetch();
        self.temp = (self.fetched as u16) + 1;
        self.write(self.addr_abs, (self.temp & 0x00FF) as u8);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, (self.temp & 0x0080) != 0); //Should be ok??
        return 0 as u8;
    } 
    pub fn SBC(&mut self) -> u8{
        self.fetch();

        let value: u16 = (self.fetched as u16) ^ 0x00FF;
        
        self.temp = self.a as u16 + value as u16 + (if self.get_flag(Flag::C) {1} else {0});

        self.set_flag(Flag::C, (self.temp & 0xFF00) != 0);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::V, ((self.temp ^ (self.a as u16)) & (self.temp ^ value) & (0x80)) != 0); //Be weary again

        self.set_flag(Flag::N, (self.temp & 0x0080) != 0); //Should be ok??
        self.a = (self.temp & 0x00FF) as u8;
        return 1 as u8;
    }

    pub fn ROR(&mut self) -> u8{ 
        self.fetch();
        self.temp = (((if self.get_flag(Flag::C) {1} else {0}) << 7) | (self.fetched >> 1)) as u16;
        self.set_flag(Flag::C, (self.fetched & 0x01) != 0);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, (self.temp & 0x0080) != 0); //Should be ok??
        if self.instrucs[self.operation as usize].addr_mode == AddrMode::IMP{
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (self.temp & 0x00FF) as u8);
        }
        return 0 as u8;
    }
    pub fn PLA(&mut self) -> u8{ 
        self.stkp = self.stkp + 1;
        self.a = self.read(0x0100 + self.stkp as u16);
        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, self.a >> 7 != 0);
        return 0 as u8;
    }
    pub fn BVS(&mut self) -> u8{
        if self.get_flag(Flag::V){
            self.cycles_remaining += 1;
            self.addr_abs = self.pc + (self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles_remaining += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0 as u8;
    }

    pub fn BMI(&mut self) -> u8{
        if self.get_flag(Flag::N){
            self.cycles_remaining += 1;
            self.addr_abs = self.pc + (self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles_remaining += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0 as u8;
    }

    pub fn SEI(&mut self) -> u8{ 
        self.set_flag(Flag::I, true);
        return 0 as u8;
    }
    pub fn STA(&mut self) -> u8{
        self.write(self.addr_abs, self.a);
        return 0 as u8;
    }
    pub fn STY(&mut self) -> u8{
        self.write(self.addr_abs, self.y);
        return 0 as u8;
    }

    pub fn STX(&mut self) -> u8{
        self.write(self.addr_abs, self.x);
        return 0 as u8;
    }

    pub fn DEY(&mut self) -> u8{
        self.y = self.y - 1;
        self.set_flag(Flag::Z, self.y == 0x00);
        self.set_flag(Flag::N, self.y >> 7 != 0);
        return 0 as u8;
    }
    pub fn TXA(&mut self) -> u8{
        self.a = self.x;
        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, self.a >> 7 != 0);
        return 0 as u8;
    }

    pub fn BCC(&mut self) -> u8{
        if !self.get_flag(Flag::C){
            self.cycles_remaining += 1;
            self.addr_abs = self.pc + (self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles_remaining += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0 as u8;
    }
    pub fn TYA(&mut self) -> u8{
        self.a = self.y;
        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, self.a >> 7 != 0);
        return 0 as u8;
    }
    pub fn TXS(&mut self) -> u8{
        self.stkp = self.x;
        return 0 as u8;
    }

    pub fn LDA(&mut self) -> u8{ 
        self.fetch();
        self.a = self.fetched;
        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, self.a >> 7 != 0);
        return 1 as u8;
    }

    pub fn LDX(&mut self) -> u8{ 
        self.fetch();
        self.x = self.fetched;
        self.set_flag(Flag::Z, self.x == 0x00);
        self.set_flag(Flag::N, self.x >> 7 != 0);
        return 1 as u8;
    }

    pub fn LDY(&mut self) -> u8{
        self.fetch();
        self.y = self.fetched;
        self.set_flag(Flag::Z, self.y == 0x00);
        self.set_flag(Flag::N, self.y >> 7 != 0);
        return 1 as u8;
    }


    pub fn TAY(&mut self) -> u8{
        self.y = self.a;
        self.set_flag(Flag::Z, self.y == 0x00);
        self.set_flag(Flag::N, self.y >> 7 != 0);
        return 0 as u8;
    }
    pub fn TAX(&mut self) -> u8{ 
        self.x = self.a;
        self.set_flag(Flag::Z, self.x == 0x00);
        self.set_flag(Flag::N, self.x >> 7 != 0);
        return 0 as u8;
    }

    pub fn BCS(&mut self) -> u8{
        if self.get_flag(Flag::C){
            self.cycles_remaining += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles_remaining += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0 as u8;

    }

    pub fn BPL(&mut self) -> u8{
        if !self.get_flag(Flag::N) {
            self.cycles_remaining += 1;
            self.addr_abs = self.pc + (self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles_remaining += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0 as u8;
    }

    pub fn CLV(&mut self) -> u8{
        self.set_flag(Flag::V, false);
        return 0 as u8;
    }

    pub fn TSX(&mut self) -> u8{
        self.x = self.stkp;
        self.set_flag(Flag::Z, self.x == 0x00);
        self.set_flag(Flag::N, self.x >> 7 != 0);
        return 0 as u8;
    }

    pub fn CPX(&mut self) -> u8{
        self.fetch();
        self.temp = self.x as u16 - self.fetched as u16;
        self.set_flag(Flag::C, self.x >= self.fetched);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, ((1 << 7) as u16 & self.temp) != 0); //Should be ok??
        return 0 as u8;
    }
    pub fn CPY(&mut self) -> u8{
        self.fetch();
        self.temp = self.y as u16 - self.fetched as u16;
        self.set_flag(Flag::C, self.y >= self.fetched);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, ((1 << 7) as u16 & self.temp) != 0); //Should be ok??
        return 0 as u8;
    }
    pub fn CMP(&mut self) -> u8{
        self.fetch();
        self.temp = self.a as u16 - self.fetched as u16;
        self.set_flag(Flag::C, self.a >= self.fetched);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, ((1 << 7) as u16 & self.temp) != 0); //Should be ok??
        return 1 as u8;
    }

    pub fn DEC(&mut self) -> u8{
        self.fetch();
        self.temp = (self.fetched as u16) - 1;
        self.write(self.addr_abs, self.temp as u8);
        self.set_flag(Flag::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flag::N, ((1 << 7) as u16 & self.temp) != 0); //Should be ok??
        return 0 as u8;
    }
    pub fn DEX(&mut self) -> u8{
        self.x -= 1;
        self.set_flag(Flag::Z, self.x == 0x00);
        self.set_flag(Flag::N, self.x >> 7 != 0);
        return 0 as u8;
    }

    pub fn BNE(&mut self) -> u8{
        if !self.get_flag(Flag::Z){
            self.cycles_remaining += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles_remaining += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0 as u8;
    }
    pub fn CLD(&mut self) -> u8{
        self.set_flag(Flag::D, false);
        return 0 as u8;
    }
    pub fn INX(&mut self) -> u8{
        self.x = self.x + 1;
        self.set_flag(Flag::Z, self.x == 0x00);
        self.set_flag(Flag::N, self.x >> 7 != 0);
        return 0 as u8;
    }
    pub fn INY(&mut self) -> u8{
        self.y = self.y + 1;
        self.set_flag(Flag::Z, self.y == 0x00);
        self.set_flag(Flag::N, self.y >> 7 != 0);
        return 0 as u8;
    }

    pub fn BEQ(&mut self) -> u8{
        if self.get_flag(Flag::Z){
            self.cycles_remaining += 1;
            self.addr_abs = self.pc + (self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles_remaining += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0 as u8;
    }
    pub fn SED(&mut self) -> u8{ 
        self.set_flag(Flag::D, true);
        return 0 as u8;
    }

    pub fn clock(&mut self){
        if self.cycles_remaining == 0{
            //println!("{}", self.register_string());
            self.operation = self.read(self.pc);
            let log_pc = self.pc;

            self.set_flag(Flag::U, true);
            self.pc = self.pc + 1;
            self.cycles_remaining = self.instrucs[self.operation as usize].n_cycles;



            let a = self.call_addr_mode(self.instrucs[self.operation as usize].addr_mode);

            let b = self.call_op(self.instrucs[self.operation as usize].operation);

            self.cycles_remaining += a&b;
            self.set_flag(Flag::U, true); //Not needed?

            //unsafe{(*(*self.data_bus).ppu).t(); }
            //println!("{}: {:#04x}: {}, {}   cycles: ({}) + A{} + O{}, x: {}",&self.clock_count,log_pc, self.instrucs[self.operation as usize].operation, self.instrucs[self.operation as usize].addr_mode,self.instrucs[self.operation as usize].n_cycles, a,b, self.x);

        }
        self.cycles_remaining -= 1;

        //self.clock_count += 1;
    }

    pub fn nmi(&mut self) {
        self.write(0x0100 + self.stkp as u16, (self.pc >> 8) as u8);
        self.stkp -= 1;
        self.write(0x0100 + self.stkp as u16, self.pc as u8);
        self.stkp -= 1;

        self.set_flag(Flag::B, false);
        self.set_flag(Flag::U, true);
        self.set_flag(Flag::I, true);
        self.write(0x0100 + self.stkp as u16, self.flags);
        self.stkp -= 1;

        self.addr_abs = 0xFFFA;
        let lo: u16 = self.read(self.addr_abs + 0) as u16;
        let hi: u16 = self.read(self.addr_abs + 1) as u16;
        self.pc = (hi << 8) | lo;

        self.cycles_remaining = 8;
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool){
        if value{
            self.flags |= flag as u8;
        } else {
            self.flags &= !(flag as u8);
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool{
        return (flag as u8 & self.flags) != 0;
    }
}