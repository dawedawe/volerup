use crate::floating::Floating;

pub mod floating;

pub struct Cpu {
    pub general_purpose_registers: [u8; 16],
    pub main_memory: [u8; 256],
    pub program_counter: usize,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            general_purpose_registers: [0; 16],
            main_memory: [0; 256],
            program_counter: 0,
            halted: false,
        }
    }

    pub fn init(program: &[u8]) -> Self {
        let mut cpu = Cpu::new();
        if program.len() > cpu.main_memory.len() {
            panic!("given program does not fit into memory");
        }
        cpu.main_memory[..program.len()].copy_from_slice(program);
        cpu
    }

    pub fn fetch(&self) -> Instruction {
        let instr_byte0 = self.main_memory[self.program_counter];
        let instr_byte1 = self.main_memory[self.program_counter + 1];
        let mut instr: u16 = (instr_byte0 as u16) << 8;
        instr |= instr_byte1 as u16;
        Instruction { instr }
    }

    fn decode(&self, instr: Instruction) -> OpCodes {
        let opcode_bits = instr.get_opcode_bits();
        let operand1 = instr.get_operand1_bits();
        let operand2 = instr.get_operand2_bits();
        let operand3 = instr.get_operand3_bits();
        match opcode_bits {
            0x1 => OpCodes::LoadAddr {
                reg: operand1,
                addr: instr.get_operand23_bits(),
            },
            0x2 => OpCodes::LoadValue {
                reg: operand1,
                value: instr.get_operand23_bits(),
            },
            0x3 => OpCodes::Store {
                reg: operand1,
                addr: instr.get_operand23_bits(),
            },
            0x4 => OpCodes::Move {
                source_reg: operand2,
                target_reg: operand3,
            },
            0x5 => OpCodes::AddInt {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            },
            0x6 => OpCodes::AddFloat {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            },
            0x7 => OpCodes::Or {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            },
            0x8 => OpCodes::And {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            },
            0x9 => OpCodes::Xor {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            },
            0xA => OpCodes::Rotate {
                reg: operand1,
                times: operand3,
            },
            0xB => OpCodes::Jump {
                reg: operand1,
                addr: instr.get_operand23_bits(),
            },
            0xC => OpCodes::Halt,
            _ => todo!(),
        }
    }

    fn execute(&mut self, opcode: OpCodes) {
        match opcode {
            OpCodes::LoadAddr { reg, addr: address } => {
                self.general_purpose_registers[reg as usize] = self.main_memory[address as usize];
            }
            OpCodes::LoadValue { reg, value } => {
                self.general_purpose_registers[reg as usize] = value;
            }
            OpCodes::Store { reg, addr } => {
                self.main_memory[addr as usize] = self.general_purpose_registers[reg as usize];
            }
            OpCodes::Move {
                source_reg,
                target_reg,
            } => {
                self.general_purpose_registers[target_reg as usize] =
                    self.general_purpose_registers[source_reg as usize];
            }
            OpCodes::AddInt {
                target_reg,
                reg1,
                reg2,
            } => {
                self.general_purpose_registers[target_reg as usize] = self
                    .general_purpose_registers[reg1 as usize]
                    + self.general_purpose_registers[reg2 as usize];
            }
            OpCodes::AddFloat {
                target_reg,
                reg1,
                reg2,
            } => {
                let f1 = Floating {
                    value: self.general_purpose_registers[reg1 as usize],
                };
                let f2 = Floating {
                    value: self.general_purpose_registers[reg2 as usize],
                };
                let sum = f1.decode() + f2.decode();
                let f3 = Floating::encode(sum);
                self.general_purpose_registers[target_reg as usize] = f3.value
            }
            OpCodes::Or {
                target_reg,
                reg1,
                reg2,
            } => {
                self.general_purpose_registers[target_reg as usize] = self
                    .general_purpose_registers[reg1 as usize]
                    | self.general_purpose_registers[reg2 as usize];
            }
            OpCodes::And {
                target_reg,
                reg1,
                reg2,
            } => {
                self.general_purpose_registers[target_reg as usize] = self
                    .general_purpose_registers[reg1 as usize]
                    & self.general_purpose_registers[reg2 as usize];
            }
            OpCodes::Xor {
                target_reg,
                reg1,
                reg2,
            } => {
                self.general_purpose_registers[target_reg as usize] = self
                    .general_purpose_registers[reg1 as usize]
                    ^ self.general_purpose_registers[reg2 as usize];
            }
            OpCodes::Rotate { reg, times } => {
                self.general_purpose_registers[reg as usize] =
                    self.general_purpose_registers[reg as usize].rotate_right(times as u32)
            }
            OpCodes::Jump { reg, addr } => {
                if self.general_purpose_registers[0] == self.general_purpose_registers[reg as usize]
                {
                    self.program_counter = self.main_memory[addr as usize] as usize;
                }
            }
            OpCodes::Halt => {
                self.halted = true;
            }
        }
    }

    pub fn step(&mut self) {
        let next_instruction = self.fetch();
        let opcode = self.decode(next_instruction);
        self.execute(opcode);
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Instruction {
    pub instr: u16,
}

impl Instruction {
    pub fn get_opcode_bits(&self) -> u8 {
        (self.instr >> 12) as u8
    }

    pub fn get_operand1_bits(&self) -> u8 {
        ((self.instr & 0x0F00) >> 8) as u8
    }

    pub fn get_operand2_bits(&self) -> u8 {
        ((self.instr & 0x00F0) >> 4) as u8
    }

    pub fn get_operand3_bits(&self) -> u8 {
        (self.instr & 0x000F) as u8
    }

    pub fn get_operand23_bits(&self) -> u8 {
        (self.instr & 0x00FF) as u8
    }

    pub fn get_operand_bits(&self) -> u16 {
        self.instr & 0x0FFF
    }
}

pub enum OpCodes {
    LoadAddr { reg: u8, addr: u8 },                  // 0x1
    LoadValue { reg: u8, value: u8 },                // 0x2
    Store { reg: u8, addr: u8 },                     // 0x3
    Move { source_reg: u8, target_reg: u8 },         // 0x4
    AddInt { target_reg: u8, reg1: u8, reg2: u8 },   // 0x5
    AddFloat { target_reg: u8, reg1: u8, reg2: u8 }, // 0x6
    Or { target_reg: u8, reg1: u8, reg2: u8 },       // 0x7
    And { target_reg: u8, reg1: u8, reg2: u8 },      // 0x8
    Xor { target_reg: u8, reg1: u8, reg2: u8 },      // 0x9
    Rotate { reg: u8, times: u8 },                   // 0xA
    Jump { reg: u8, addr: u8 },                      // 0xB
    Halt,                                            // 0xC
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn get_opcode_bits_works() {
        let i = Instruction { instr: 0x1234 };
        assert_eq!(i.get_opcode_bits(), 0x01)
    }

    #[test]
    pub fn get_operand_bits_works() {
        let i = Instruction { instr: 0x1234 };
        assert_eq!(i.get_operand_bits(), 0x0234)
    }

    #[test]
    pub fn opcode_loadaddr_works() {
        let program = [0x14, 0xA3];
        let mut cpu = Cpu::init(&program);
        cpu.main_memory[0xA3] = 0xCD;
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x04], 0xCD)
    }

    #[test]
    pub fn opcode_loadvalue_works() {
        let program = [0x20, 0xA3];
        let mut cpu = Cpu::init(&program);
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x00], 0xA3)
    }

    #[test]
    pub fn opcode_store_works() {
        let program = [0x35, 0xB1];
        let mut cpu = Cpu::init(&program);
        cpu.general_purpose_registers[5] = 0x58;
        cpu.step();
        assert_eq!(cpu.main_memory[0xB1], 0x58)
    }

    #[test]
    pub fn opcode_move_works() {
        let program = [0x40, 0xA4];
        let mut cpu = Cpu::init(&program);
        cpu.general_purpose_registers[0xA] = 0xFF;
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x04], 0xFF)
    }

    #[test]
    pub fn opcode_addint_works() {
        let program = [0x57, 0x26];
        let mut cpu = Cpu::init(&program);
        cpu.general_purpose_registers[0x02] = 0x03;
        cpu.general_purpose_registers[0x06] = 0x05;
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x07], 0x08)
    }

    #[test]
    pub fn opcode_addfloat_works() {
        let program = [0x63, 0x4E];
        let mut cpu = Cpu::init(&program);
        cpu.general_purpose_registers[0x04] = 0x03;
        cpu.general_purpose_registers[0x0E] = 0x05;
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x03], 0x08)
    }

    #[test]
    pub fn opcode_or_works() {
        let program = [0x7C, 0xB4];
        let mut cpu = Cpu::init(&program);
        cpu.general_purpose_registers[0x0B] = 0xF0;
        cpu.general_purpose_registers[0x04] = 0x0F;
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x0C], 0xFF)
    }

    #[test]
    pub fn opcode_and_works() {
        let program = [0x80, 0x45];
        let mut cpu = Cpu::init(&program);
        cpu.general_purpose_registers[0x04] = 0xF3;
        cpu.general_purpose_registers[0x05] = 0x05;
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x00], 0x01)
    }

    #[test]
    pub fn opcode_xor_works() {
        let program = [0x95, 0xF3];
        let mut cpu = Cpu::init(&program);
        cpu.general_purpose_registers[0x0F] = 0xF3;
        cpu.general_purpose_registers[0x03] = 0x05;
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x05], 0xF6)
    }

    #[test]
    pub fn opcode_rotate_works() {
        let program = [0xA4, 0x03];
        let mut cpu = Cpu::init(&program);
        cpu.general_purpose_registers[0x04] = 0x07;
        cpu.step();
        assert_eq!(cpu.general_purpose_registers[0x04], 0xE0)
    }

    #[test]
    pub fn opcode_jump_works() {
        let program = [0xB4, 0x3C];
        let mut cpu = Cpu::init(&program);
        cpu.main_memory[0x3C] = 0xAB;
        cpu.general_purpose_registers[0x00] = 0x05;
        cpu.general_purpose_registers[0x04] = 0x05;
        cpu.step();
        assert_eq!(cpu.program_counter, 0xAB)
    }

    #[test]
    pub fn opcode_halt_works() {
        let program = [0xC0];
        let mut cpu = Cpu::init(&program);
        cpu.step();
        assert!(cpu.halted)
    }
}
