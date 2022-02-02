mod instructions;
use instructions as insts;
pub struct Cpu {
    pub memory: [u8; 4096],
    pub register: [u8; 16],
    pub pc: usize,
    pub sp: usize,
    pub stack: [usize; 16],
    pub i_reg: u16
}
pub fn init() -> Cpu {
    println!("Initializing CPU...");
    return Cpu {memory:[0; 4096],
                register: [0; 16],
                pc: 0x200,
                sp: 0,
                stack: [0; 16],
                i_reg: 0,
    };
}
impl Cpu {
    pub fn load_rom(&mut self, rom_file: String) {
        println!("Loading ROM...");
        let data = std::fs::read(rom_file).unwrap();
        for (i,val) in data.iter().enumerate(){
            self.memory[i+0x200] = *val;
        }
    }
    pub fn fetch_instruction(&mut self){
        let inst_half1:u16 = self.memory[self.pc as usize] as u16;
        let inst_half2:u16 = self.memory[(self.pc + 1) as usize] as u16;
        let nibble1 = (inst_half1 & 0xF0) >> 4;
        let nibble2 = inst_half1 & 0x0F;
        let nibble3 = (inst_half2 & 0xF0) >> 4;
        let nibble4 = inst_half2 & 0x0F;

        self.execute_instruction((nibble1, nibble2, nibble3, nibble4));
    }
    pub fn execute_instruction(&mut self, inst: (u16, u16, u16, u16)){
        match inst{
            (0x0, 0x0, 0xE, 0xE) =>  insts::ret(self),
            (0x1, a, b, c) =>  insts::jp(self, (a << 8 | b << 4 | c) as u16),
            (0x2, a, b, c) =>  insts::call(self, (a << 8 | b << 4 | c) as u16),
            (0x3, a, b, c) =>  insts::skip_eq(self, a as usize, (b << 4 | c) as u16),
            (0x4, a, b, c) =>  insts::skip_not_eq(self, a as usize, (b << 4 | c) as u16),
            (0x5, a, b, 0x0) =>  insts::skip_reg_eq(self, a as usize, b as usize),
            (0x6, a, b, c) =>  insts::load_immediate(self, a as usize, (b << 4 | c) as u8),
            (a,b,c,d) => {println!("Undefined opcode {}{}{}{}", a,b,c,d); insts::nop(self);}
        }
    }

}