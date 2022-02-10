mod instructions;
use instructions as insts;
use crate::font_data;
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum KeyState{
    Pressed,
    NotPressed
}

pub struct Cpu {
    pub memory: [u8; 4096],
    pub register: [u8; 16],
    pub pc: usize,
    pub sp: usize,
    pub stack: [usize; 16],
    pub i_reg: u16,
    pub frame_buffer: [u8; 64*32],
    pub key_states: [KeyState; 16],
    pub delay_timer: u8,
}
pub fn init() -> Cpu {
    println!("Initializing CPU...");
    return Cpu {memory:[0; 4096],
                register: [0; 16],
                pc: 0x200,
                sp: 0,
                stack: [0; 16],
                i_reg: 0,
                frame_buffer: [0; 64*32],
                key_states: [KeyState::NotPressed; 16],
                delay_timer: 0,
    };
}
impl Cpu {
    pub fn load_rom(&mut self, rom_file: String) {
        println!("Loading ROM...");
        let data = std::fs::read(rom_file).unwrap();
        for (i,val) in data.iter().enumerate(){
            self.memory[i+0x200] = *val;
        }


        for i in 0..5*16{
            self.memory[i] = font_data::FONT_DATA[i];
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
            (0x0, 0x0, 0x0, 0x0) =>  insts::nop(self),
            (0x0, 0x0, 0xE, 0x0) =>  insts::cls(self),
            (0x0, 0x0, 0xE, 0xE) =>  insts::ret(self),
            (0x1, a, b, c) =>  insts::jp(self, (a << 8 | b << 4 | c) as u16),
            (0x2, a, b, c) =>  insts::call(self, (a << 8 | b << 4 | c) as u16),
            (0x3, a, b, c) =>  insts::skip_eq(self, a as usize, (b << 4 | c) as u16),
            (0x4, a, b, c) =>  insts::skip_not_eq(self, a as usize, (b << 4 | c) as u16),
            (0x5, a, b, 0x0) =>  insts::skip_reg_eq(self, a as usize, b as usize),
            (0x6, a, b, c) =>  insts::load_immediate(self, a as usize, (b << 4 | c) as u8),
            (0x7, a, b, c) =>  insts::add_immediate(self, a as usize, (b << 4 | c) as u8),
            (0x8, a, b, 0x0) =>  insts::load_reg(self, a as usize, b as usize),
            (0x8, a, b, 0x1) =>  insts::or_reg(self, a as usize, b as usize),
            (0x8, a, b, 0x2) =>  insts::and_reg(self, a as usize, b as usize),
            (0x8, a, b, 0x3) =>  insts::xor_reg(self, a as usize, b as usize),
            (0x8, a, b, 0x4) =>  insts::add_reg(self, a as usize, b as usize),
            (0x8, a, b, 0x5) =>  insts::sub_reg(self, a as usize, b as usize),
            (0x8, a, _, 0x6) =>  insts::shr_reg(self, a as usize),
            (0x8, a, b, 0x7) =>  insts::sub_n_reg(self, a as usize, b as usize),
            (0x8, a, _, 0xE) =>  insts::shl_reg(self, a as usize),
            (0x9, a, b, 0x0) =>  insts::skip_reg_not_eq(self, a as usize, b as usize),
            (0xA, a, b, c) =>  insts::load_immediate_i(self, (a << 8 | b << 4 | c) as u16),
            (0xB, a, b, c) =>  insts::jp_v0(self, (a << 8 | b << 4 | c) as u16),
            (0xC, a, b, c) =>  insts::rnd(self, a as usize, (b << 4 | c) as u16),
            (0xD, a, b, c) =>  insts::draw(self, a as usize, b as usize, c as u16),
            (0xE, a, 0x9, 0xE) =>  insts::skip_pressed(self, a),
            (0xE, a, 0xA, 0x1) =>  insts::skip_not_pressed(self, a),
            (0xF, a, 0x0, 0x7) =>  insts::read_delay_timer(self, a),
            (0xF, a, 0x1, 0x5) =>  insts::set_delay_timer(self, a),
            (0xF, a, 0x1, 0xE) =>  insts::add_i(self, a),
            (0xF, a, 0x2, 0x9) =>  insts::load_sprite(self, a),
            (0xF, a, 0x3, 0x3) =>  insts::load_bcd(self, a),
            (0xF, a, 0x5, 0x5) =>  insts::store_x_regs(self, a),
            (0xF, a, 0x6, 0x5) =>  insts::load_x_regs(self, a),

            (a,b,c,d) => {println!("Undefined opcode {} {} {} {} at PC={}", a,b,c,d, self.pc); insts::nop(self);}
        }
    }

}