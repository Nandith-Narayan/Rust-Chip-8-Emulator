use crate::cpu::{Cpu, KeyState};
use rand::Rng;

pub fn nop(cpu:&mut Cpu){
    cpu.pc+=2;
}
pub fn cls(cpu:&mut Cpu){
    for i in 0..64*32 {
        cpu.frame_buffer[i] = 0;
    }
    cpu.pc += 2;
}
pub fn ret(cpu:&mut Cpu){
    cpu.sp -= 1;
    cpu.pc = cpu.stack[cpu.sp];
}
pub fn jp(cpu:&mut Cpu, address: u16){
    cpu.pc = address as usize;

}
pub fn call(cpu:&mut Cpu, address: u16){

    cpu.stack[cpu.sp] = cpu.pc+2;
    cpu.sp += 1;
    cpu.pc = address as usize;

}
pub fn skip_eq(cpu:&mut Cpu, reg:usize, val:u16){
    if cpu.register[reg] == val as u8 {
        cpu.pc += 4;
    }else{
        cpu.pc += 2;
    }
}
pub fn skip_not_eq(cpu:&mut Cpu, reg:usize, val:u16){
    if cpu.register[reg] != val as u8 {
        cpu.pc += 4;
    }else{
        cpu.pc += 2;
    }
}
pub fn skip_reg_eq(cpu:&mut Cpu, reg1:usize, reg2:usize){
    if cpu.register[reg1] == cpu.register[reg2]  {
        cpu.pc += 4;
    }else{
        cpu.pc += 2;
    }
}
pub fn load_immediate(cpu:&mut Cpu, reg:usize, val:u8){
    cpu.register[reg] = val;
    cpu.pc += 2;
}
pub fn add_immediate(cpu:&mut Cpu, reg:usize, val:u8){
    cpu.register[reg] = (cpu.register[reg]+val) & 0xFF;
    cpu.pc += 2;
}
pub fn load_reg(cpu:&mut Cpu, reg1:usize, reg2:usize){
    cpu.register[reg1] = cpu.register[reg2];
    cpu.pc += 2;
}
pub fn or_reg(cpu:&mut Cpu, reg1:usize, reg2:usize){
    cpu.register[reg1] |= cpu.register[reg2];
    cpu.pc += 2;
}
pub fn and_reg(cpu:&mut Cpu, reg1:usize, reg2:usize){
    cpu.register[reg1] &= cpu.register[reg2];
    cpu.pc += 2;
}
pub fn xor_reg(cpu:&mut Cpu, reg1:usize, reg2:usize) {
    cpu.register[reg1] ^= cpu.register[reg2];
    cpu.pc += 2;
}
pub fn add_reg(cpu:&mut Cpu, reg1:usize, reg2:usize){
    let value = cpu.register[reg1] as u16 + cpu.register[reg2] as u16;
    if value > 0xFF{
        cpu.register[0xF] = 1;
    }else{
        cpu.register[0xF] = 0;
    }
    cpu.register[reg1] = value as u8;
    cpu.pc += 2;
}
pub fn sub_reg(cpu:&mut Cpu, reg1:usize, reg2:usize){
    let value = cpu.register[reg1] as u16 - cpu.register[reg2] as u16;
    if cpu.register[reg1] > cpu.register[reg2]{
        cpu.register[0xF] = 1;
    }else{
        cpu.register[0xF] = 0;
    }
    cpu.register[reg1] = value as u8;
    cpu.pc += 2;
}
pub fn shr_reg(cpu:&mut Cpu, reg1:usize){
    cpu.register[0xF] = cpu.register[reg1]&0x1;
    cpu.register[reg1] >>= 1;
    cpu.pc += 2;
}
pub fn sub_n_reg(cpu:&mut Cpu, reg1:usize, reg2:usize){
    let value = cpu.register[reg2] as u16 - cpu.register[reg1] as u16;
    if cpu.register[reg1] < cpu.register[reg2]{
        cpu.register[0xF] = 1;
    }else{
        cpu.register[0xF] = 0;
    }
    cpu.register[reg1] = value as u8;
    cpu.pc += 2;
}
pub fn shl_reg(cpu:&mut Cpu, reg1:usize){
    cpu.register[0xF] = (cpu.register[reg1]>>7)&0x1;
    cpu.register[reg1] <<= 1;
    cpu.pc += 2;
}
pub fn skip_reg_not_eq(cpu:&mut Cpu, reg1:usize, reg2:usize){
    if cpu.register[reg1] != cpu.register[reg2]  {
        cpu.pc += 4;
    }else{
        cpu.pc += 2;
    }
}
pub fn load_immediate_i(cpu:&mut Cpu, val:u16){
    cpu.i_reg = val;
    cpu.pc += 2;
}
pub fn jp_v0(cpu:&mut Cpu, address: u16){ cpu.pc = address as usize + (cpu.register[0] as usize);
}
pub fn rnd(cpu:&mut Cpu, reg:usize, val:u16){
    let mut rng = rand::thread_rng();
    let r:u8 = rng.gen();
    cpu.register[reg] = r & (val as u8);
    cpu.pc += 2;
}
pub fn draw(cpu:&mut Cpu, reg1:usize, reg2:usize, val:u16){
    // TODO: set VF on erase
    let start_address = cpu.i_reg;
    let start_address_frame = (cpu.register[reg1] as u16) + ((cpu.register[reg2] as u16)*64);
    cpu.register[0xF] = 0;

    for offset in 0..val{
        let byte = cpu.memory[(offset+start_address) as usize];
        for i in 0..8{
            let color = (byte >> i)&0x1;
            let address = start_address_frame+(offset*64)+(7-i);
            if address < 2048 {
                let old = cpu.frame_buffer[address as usize];
                let new = cpu.frame_buffer[address as usize] ^ color;
                cpu.frame_buffer[address as usize] = new;
                if old==1 && new == 0{
                    cpu.register[0xF] = 1;
                }
            }
        }
    }
    cpu.pc += 2;
}
pub fn skip_pressed(cpu:&mut Cpu, reg:u16){
    if cpu.key_states[reg as usize] == KeyState::Pressed{
        cpu.pc += 4;
    }else {
        cpu.pc += 2;
    }
}
pub fn skip_not_pressed(cpu:&mut Cpu, reg:u16){
    if cpu.key_states[reg as usize] == KeyState::NotPressed{
        cpu.pc += 4;
    }else {
        cpu.pc += 2;
    }
}
pub fn read_delay_timer(cpu:&mut Cpu, reg:u16){
    cpu.register[reg as usize] = cpu.delay_timer;
    cpu.pc += 2;
}
pub fn set_delay_timer(cpu:&mut Cpu, reg:u16){
    cpu.delay_timer =cpu.register[reg as usize];
    cpu.pc += 2;
}
pub fn add_i(cpu:&mut Cpu, reg:u16){
    cpu.i_reg += cpu.register[reg as usize] as u16;
    cpu.pc += 2;
}
pub fn load_sprite(cpu:&mut Cpu, reg:u16){
    cpu.i_reg = (cpu.register[reg as usize] * 5) as u16;
    cpu.pc += 2;
}
pub fn load_bcd(cpu:&mut Cpu, reg:u16){
    let val = cpu.register[reg as usize];
    cpu.memory[cpu.i_reg as usize] = val/100;
    cpu.memory[(cpu.i_reg+1) as usize] = (val%100)/10;
    cpu.memory[(cpu.i_reg+2) as usize] = val%10;
    cpu.pc += 2;
}
pub fn store_x_regs(cpu:&mut Cpu, reg:u16){
    for i in 0..(reg+1){
        cpu.memory[(cpu.i_reg+i) as usize] = cpu.register[i as usize];
    }
    cpu.pc += 2;
}
pub fn load_x_regs(cpu:&mut Cpu, reg:u16){
    for i in 0..(reg+1){
        cpu.register[i as usize] = cpu.memory[(cpu.i_reg+i) as usize];
    }
    cpu.pc += 2;
}
