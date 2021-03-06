pub struct Instruction{
    pub name: String,
    pub operation: Operation,
    pub addr_mode: AddrMode,
    pub n_cycles: u8
}
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum AddrMode{
    ZPX,
    IND,
    IMM,
    IZY,
    ABY,
    ABS,
    IMP,
    ABX,
    ZP0,
    REL,
    IZX,
    ZPY,
}
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Operation{
    LSR,
    CPY,
    CPX,
    BEQ,
    JMP,
    TSX,
    RTS,
    XXX,
    INC,
    LDX,
    CLD,
    BCC,
    INX,
    BMI,
    ADC,
    BPL,
    STY,
    CMP,
    ORA,
    EOR,
    DEC,
    RTI,
    BIT,
    ASL,
    STX,
    STA,
    BCS,
    SEI,
    JSR,
    PHP,
    TXS,
    CLV,
    ROR,
    CLC,
    SBC,
    PHA,
    SEC,
    LDA,
    DEX,
    ROL,
    TAY,
    CLI,
    PLP,
    TAX,
    PLA,
    BVC,
    BNE,
    TYA,
    INY,
    NOP,
    BRK,
    BVS,
    AND,
    DEY,
    TXA,
    LDY,
    SED,
}

pub fn get_instruction_defs() -> [Instruction; 256]{
    let instruction_defs: [Instruction; 256] = [
        Instruction{name: String::from("BRK"), operation: Operation::BRK, addr_mode: AddrMode::IMM, n_cycles: 7},
        Instruction{name: String::from("ORA"), operation: Operation::ORA, addr_mode: AddrMode::IZX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 3},
        Instruction{name: String::from("ORA"), operation: Operation::ORA, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("ASL"), operation: Operation::ASL, addr_mode: AddrMode::ZP0, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("PHP"), operation: Operation::PHP, addr_mode: AddrMode::IMP, n_cycles: 3},
        Instruction{name: String::from("ORA"), operation: Operation::ORA, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("ASL"), operation: Operation::ASL, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("ORA"), operation: Operation::ORA, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("ASL"), operation: Operation::ASL, addr_mode: AddrMode::ABS, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("BPL"), operation: Operation::BPL, addr_mode: AddrMode::REL, n_cycles: 2},
        Instruction{name: String::from("ORA"), operation: Operation::ORA, addr_mode: AddrMode::IZY, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("ORA"), operation: Operation::ORA, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("ASL"), operation: Operation::ASL, addr_mode: AddrMode::ZPX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("CLC"), operation: Operation::CLC, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("ORA"), operation: Operation::ORA, addr_mode: AddrMode::ABY, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("ORA"), operation: Operation::ORA, addr_mode: AddrMode::ABX, n_cycles: 4},
        Instruction{name: String::from("ASL"), operation: Operation::ASL, addr_mode: AddrMode::ABX, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("JSR"), operation: Operation::JSR, addr_mode: AddrMode::ABS, n_cycles: 6},
        Instruction{name: String::from("AND"), operation: Operation::AND, addr_mode: AddrMode::IZX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("BIT"), operation: Operation::BIT, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("AND"), operation: Operation::AND, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("ROL"), operation: Operation::ROL, addr_mode: AddrMode::ZP0, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("PLP"), operation: Operation::PLP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("AND"), operation: Operation::AND, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("ROL"), operation: Operation::ROL, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("BIT"), operation: Operation::BIT, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("AND"), operation: Operation::AND, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("ROL"), operation: Operation::ROL, addr_mode: AddrMode::ABS, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("BMI"), operation: Operation::BMI, addr_mode: AddrMode::REL, n_cycles: 2},
        Instruction{name: String::from("AND"), operation: Operation::AND, addr_mode: AddrMode::IZY, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("AND"), operation: Operation::AND, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("ROL"), operation: Operation::ROL, addr_mode: AddrMode::ZPX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("SEC"), operation: Operation::SEC, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("AND"), operation: Operation::AND, addr_mode: AddrMode::ABY, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("AND"), operation: Operation::AND, addr_mode: AddrMode::ABX, n_cycles: 4},
        Instruction{name: String::from("ROL"), operation: Operation::ROL, addr_mode: AddrMode::ABX, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("RTI"), operation: Operation::RTI, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("EOR"), operation: Operation::EOR, addr_mode: AddrMode::IZX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 3},
        Instruction{name: String::from("EOR"), operation: Operation::EOR, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("LSR"), operation: Operation::LSR, addr_mode: AddrMode::ZP0, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("PHA"), operation: Operation::PHA, addr_mode: AddrMode::IMP, n_cycles: 3},
        Instruction{name: String::from("EOR"), operation: Operation::EOR, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("LSR"), operation: Operation::LSR, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("JMP"), operation: Operation::JMP, addr_mode: AddrMode::ABS, n_cycles: 3},
        Instruction{name: String::from("EOR"), operation: Operation::EOR, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("LSR"), operation: Operation::LSR, addr_mode: AddrMode::ABS, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("BVC"), operation: Operation::BVC, addr_mode: AddrMode::REL, n_cycles: 2},
        Instruction{name: String::from("EOR"), operation: Operation::EOR, addr_mode: AddrMode::IZY, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("EOR"), operation: Operation::EOR, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("LSR"), operation: Operation::LSR, addr_mode: AddrMode::ZPX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("CLI"), operation: Operation::CLI, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("EOR"), operation: Operation::EOR, addr_mode: AddrMode::ABY, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("EOR"), operation: Operation::EOR, addr_mode: AddrMode::ABX, n_cycles: 4},
        Instruction{name: String::from("LSR"), operation: Operation::LSR, addr_mode: AddrMode::ABX, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("RTS"), operation: Operation::RTS, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("ADC"), operation: Operation::ADC, addr_mode: AddrMode::IZX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 3},
        Instruction{name: String::from("ADC"), operation: Operation::ADC, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("ROR"), operation: Operation::ROR, addr_mode: AddrMode::ZP0, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("PLA"), operation: Operation::PLA, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("ADC"), operation: Operation::ADC, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("ROR"), operation: Operation::ROR, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("JMP"), operation: Operation::JMP, addr_mode: AddrMode::IND, n_cycles: 5},
        Instruction{name: String::from("ADC"), operation: Operation::ADC, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("ROR"), operation: Operation::ROR, addr_mode: AddrMode::ABS, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("BVS"), operation: Operation::BVS, addr_mode: AddrMode::REL, n_cycles: 2},
        Instruction{name: String::from("ADC"), operation: Operation::ADC, addr_mode: AddrMode::IZY, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("ADC"), operation: Operation::ADC, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("ROR"), operation: Operation::ROR, addr_mode: AddrMode::ZPX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("SEI"), operation: Operation::SEI, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("ADC"), operation: Operation::ADC, addr_mode: AddrMode::ABY, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("ADC"), operation: Operation::ADC, addr_mode: AddrMode::ABX, n_cycles: 4},
        Instruction{name: String::from("ROR"), operation: Operation::ROR, addr_mode: AddrMode::ABX, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("STA"), operation: Operation::STA, addr_mode: AddrMode::IZX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("STY"), operation: Operation::STY, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("STA"), operation: Operation::STA, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("STX"), operation: Operation::STX, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 3},
        Instruction{name: String::from("DEY"), operation: Operation::DEY, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("TXA"), operation: Operation::TXA, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("STY"), operation: Operation::STY, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("STA"), operation: Operation::STA, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("STX"), operation: Operation::STX, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("BCC"), operation: Operation::BCC, addr_mode: AddrMode::REL, n_cycles: 2},
        Instruction{name: String::from("STA"), operation: Operation::STA, addr_mode: AddrMode::IZY, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("STY"), operation: Operation::STY, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("STA"), operation: Operation::STA, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("STX"), operation: Operation::STX, addr_mode: AddrMode::ZPY, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("TYA"), operation: Operation::TYA, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("STA"), operation: Operation::STA, addr_mode: AddrMode::ABY, n_cycles: 5},
        Instruction{name: String::from("TXS"), operation: Operation::TXS, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("STA"), operation: Operation::STA, addr_mode: AddrMode::ABX, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("LDY"), operation: Operation::LDY, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("LDA"), operation: Operation::LDA, addr_mode: AddrMode::IZX, n_cycles: 6},
        Instruction{name: String::from("LDX"), operation: Operation::LDX, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("LDY"), operation: Operation::LDY, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("LDA"), operation: Operation::LDA, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("LDX"), operation: Operation::LDX, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 3},
        Instruction{name: String::from("TAY"), operation: Operation::TAY, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("LDA"), operation: Operation::LDA, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("TAX"), operation: Operation::TAX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("LDY"), operation: Operation::LDY, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("LDA"), operation: Operation::LDA, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("LDX"), operation: Operation::LDX, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("BCS"), operation: Operation::BCS, addr_mode: AddrMode::REL, n_cycles: 2},
        Instruction{name: String::from("LDA"), operation: Operation::LDA, addr_mode: AddrMode::IZY, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("LDY"), operation: Operation::LDY, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("LDA"), operation: Operation::LDA, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("LDX"), operation: Operation::LDX, addr_mode: AddrMode::ZPY, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("CLV"), operation: Operation::CLV, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("LDA"), operation: Operation::LDA, addr_mode: AddrMode::ABY, n_cycles: 4},
        Instruction{name: String::from("TSX"), operation: Operation::TSX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("LDY"), operation: Operation::LDY, addr_mode: AddrMode::ABX, n_cycles: 4},
        Instruction{name: String::from("LDA"), operation: Operation::LDA, addr_mode: AddrMode::ABX, n_cycles: 4},
        Instruction{name: String::from("LDX"), operation: Operation::LDX, addr_mode: AddrMode::ABY, n_cycles: 4},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("CPY"), operation: Operation::CPY, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("CMP"), operation: Operation::CMP, addr_mode: AddrMode::IZX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("CPY"), operation: Operation::CPY, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("CMP"), operation: Operation::CMP, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("DEC"), operation: Operation::DEC, addr_mode: AddrMode::ZP0, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("INY"), operation: Operation::INY, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("CMP"), operation: Operation::CMP, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("DEX"), operation: Operation::DEX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("CPY"), operation: Operation::CPY, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("CMP"), operation: Operation::CMP, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("DEC"), operation: Operation::DEC, addr_mode: AddrMode::ABS, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("BNE"), operation: Operation::BNE, addr_mode: AddrMode::REL, n_cycles: 2},
        Instruction{name: String::from("CMP"), operation: Operation::CMP, addr_mode: AddrMode::IZY, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("CMP"), operation: Operation::CMP, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("DEC"), operation: Operation::DEC, addr_mode: AddrMode::ZPX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("CLD"), operation: Operation::CLD, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("CMP"), operation: Operation::CMP, addr_mode: AddrMode::ABY, n_cycles: 4},
        Instruction{name: String::from("NOP"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("CMP"), operation: Operation::CMP, addr_mode: AddrMode::ABX, n_cycles: 4},
        Instruction{name: String::from("DEC"), operation: Operation::DEC, addr_mode: AddrMode::ABX, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("CPX"), operation: Operation::CPX, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("SBC"), operation: Operation::SBC, addr_mode: AddrMode::IZX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("CPX"), operation: Operation::CPX, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("SBC"), operation: Operation::SBC, addr_mode: AddrMode::ZP0, n_cycles: 3},
        Instruction{name: String::from("INC"), operation: Operation::INC, addr_mode: AddrMode::ZP0, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 5},
        Instruction{name: String::from("INX"), operation: Operation::INX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("SBC"), operation: Operation::SBC, addr_mode: AddrMode::IMM, n_cycles: 2},
        Instruction{name: String::from("NOP"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::SBC, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("CPX"), operation: Operation::CPX, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("SBC"), operation: Operation::SBC, addr_mode: AddrMode::ABS, n_cycles: 4},
        Instruction{name: String::from("INC"), operation: Operation::INC, addr_mode: AddrMode::ABS, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("BEQ"), operation: Operation::BEQ, addr_mode: AddrMode::REL, n_cycles: 2},
        Instruction{name: String::from("SBC"), operation: Operation::SBC, addr_mode: AddrMode::IZY, n_cycles: 5},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 8},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("SBC"), operation: Operation::SBC, addr_mode: AddrMode::ZPX, n_cycles: 4},
        Instruction{name: String::from("INC"), operation: Operation::INC, addr_mode: AddrMode::ZPX, n_cycles: 6},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 6},
        Instruction{name: String::from("SED"), operation: Operation::SED, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("SBC"), operation: Operation::SBC, addr_mode: AddrMode::ABY, n_cycles: 4},
        Instruction{name: String::from("NOP"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 2},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::NOP, addr_mode: AddrMode::IMP, n_cycles: 4},
        Instruction{name: String::from("SBC"), operation: Operation::SBC, addr_mode: AddrMode::ABX, n_cycles: 4},
        Instruction{name: String::from("INC"), operation: Operation::INC, addr_mode: AddrMode::ABX, n_cycles: 7},
        Instruction{name: String::from("???"), operation: Operation::XXX, addr_mode: AddrMode::IMP, n_cycles: 7},
    ];

    return instruction_defs;
}

