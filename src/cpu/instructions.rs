use crate::cpu::Cpu;
pub fn nop(cpu:&mut Cpu){
    cpu.pc+=2;
}
pub fn ret(cpu:&mut Cpu){
    cpu.pc = cpu.stack[cpu.sp];
    cpu.sp -= 1;
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