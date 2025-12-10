use std::fmt::Display;

use crate::floating::Floating;

#[derive(Debug)]
pub struct Cpu {
    pub registers: [u8; 16],
    pub memory: [u8; 256],
    pub program_counter: usize,
    pub instruction_register: u16,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            memory: [0; 256],
            program_counter: 0,
            instruction_register: 0x0000,
            halted: false,
        }
    }

    pub fn init(program: &[u8]) -> Self {
        let mut cpu = Cpu::new();
        if program.len() > cpu.memory.len() {
            panic!("given program does not fit into memory");
        }
        cpu.memory[..program.len()].copy_from_slice(program);
        cpu
    }

    pub fn fetch(&mut self) {
        let instr_byte0 = self.memory[self.program_counter];
        let instr_byte1 = self.memory[self.program_counter + 1];
        let mut instr: u16 = (instr_byte0 as u16) << 8;
        instr |= instr_byte1 as u16;
        self.instruction_register = instr;
    }

    fn get_opcode_bits(instr: Instruction) -> u8 {
        (instr >> 12) as u8
    }

    pub fn get_operand1_bits(instr: Instruction) -> u8 {
        ((instr & 0x0F00) >> 8) as u8
    }

    pub fn get_operand2_bits(instr: Instruction) -> u8 {
        ((instr & 0x00F0) >> 4) as u8
    }

    pub fn get_operand3_bits(instr: Instruction) -> u8 {
        (instr & 0x000F) as u8
    }

    pub fn get_operand23_bits(instr: Instruction) -> u8 {
        (instr & 0x00FF) as u8
    }

    pub fn get_operand_bits(instr: Instruction) -> u16 {
        instr & 0x0FFF
    }

    /// Decode the bits in the instruction_register into an OpCode
    pub fn decode(&self) -> Option<OpCodes> {
        let opcode_bits = Cpu::get_opcode_bits(self.instruction_register);
        let operand1 = Cpu::get_operand1_bits(self.instruction_register);
        let operand2 = Cpu::get_operand2_bits(self.instruction_register);
        let operand3 = Cpu::get_operand3_bits(self.instruction_register);
        match opcode_bits {
            0x1 => Some(OpCodes::LoadAddr {
                reg: operand1,
                addr: Cpu::get_operand23_bits(self.instruction_register),
            }),
            0x2 => Some(OpCodes::LoadValue {
                reg: operand1,
                value: Cpu::get_operand23_bits(self.instruction_register),
            }),
            0x3 => Some(OpCodes::Store {
                reg: operand1,
                addr: Cpu::get_operand23_bits(self.instruction_register),
            }),
            0x4 => Some(OpCodes::Move {
                source_reg: operand2,
                target_reg: operand3,
            }),
            0x5 => Some(OpCodes::AddInt {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0x6 => Some(OpCodes::AddFloat {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0x7 => Some(OpCodes::Or {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0x8 => Some(OpCodes::And {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0x9 => Some(OpCodes::Xor {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0xA => Some(OpCodes::Rotate {
                reg: operand1,
                times: operand3,
            }),
            0xB => Some(OpCodes::Jump {
                reg: operand1,
                addr: Cpu::get_operand23_bits(self.instruction_register),
            }),
            0xC => Some(OpCodes::Halt),
            _ => None,
        }
    }

    fn execute(&mut self, opcode: OpCodes) {
        match opcode {
            OpCodes::LoadAddr { reg, addr: address } => {
                self.registers[reg as usize] = self.memory[address as usize];
            }
            OpCodes::LoadValue { reg, value } => {
                self.registers[reg as usize] = value;
            }
            OpCodes::Store { reg, addr } => {
                self.memory[addr as usize] = self.registers[reg as usize];
            }
            OpCodes::Move {
                source_reg,
                target_reg,
            } => {
                self.registers[target_reg as usize] = self.registers[source_reg as usize];
            }
            OpCodes::AddInt {
                target_reg,
                reg1,
                reg2,
            } => {
                self.registers[target_reg as usize] =
                    self.registers[reg1 as usize] + self.registers[reg2 as usize];
            }
            OpCodes::AddFloat {
                target_reg,
                reg1,
                reg2,
            } => {
                let f1 = Floating {
                    value: self.registers[reg1 as usize],
                };
                let f2 = Floating {
                    value: self.registers[reg2 as usize],
                };
                let sum = f1.decode() + f2.decode();
                let f3 = Floating::encode(sum);
                self.registers[target_reg as usize] = f3.value
            }
            OpCodes::Or {
                target_reg,
                reg1,
                reg2,
            } => {
                self.registers[target_reg as usize] =
                    self.registers[reg1 as usize] | self.registers[reg2 as usize];
            }
            OpCodes::And {
                target_reg,
                reg1,
                reg2,
            } => {
                self.registers[target_reg as usize] =
                    self.registers[reg1 as usize] & self.registers[reg2 as usize];
            }
            OpCodes::Xor {
                target_reg,
                reg1,
                reg2,
            } => {
                self.registers[target_reg as usize] =
                    self.registers[reg1 as usize] ^ self.registers[reg2 as usize];
            }
            OpCodes::Rotate { reg, times } => {
                self.registers[reg as usize] =
                    self.registers[reg as usize].rotate_right(times as u32)
            }
            OpCodes::Jump { reg, addr } => {
                if self.registers[0] == self.registers[reg as usize] {
                    self.program_counter = self.memory[addr as usize] as usize;
                }
            }
            OpCodes::Halt => {
                self.halted = true;
            }
        }

        match opcode {
            OpCodes::Jump { reg: _, addr: _ } => (),
            _ => self.program_counter += 2,
        }
    }

    /// Do a full fetch-decode-ececute cycle.
    /// Returns false if instruction was illegal, true otherwise.
    pub fn cycle(&mut self) -> bool {
        self.fetch();
        if let Some(opcode) = self.decode() {
            self.execute(opcode);
            true
        } else {
            self.halted = true;
            false
        }
    }

    /// Run till halt.
    /// Returns false if illegal instruction was fetched, true otherwise.
    pub fn run(&mut self) -> bool {
        let mut r = true;
        while !self.halted {
            r = self.cycle();
        }
        r
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

pub type Instruction = u16;

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

impl Display for OpCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCodes::LoadAddr { reg, addr } => write!(f, "LOADADDR 0x{:02X} 0x{:02X}", reg, addr),
            OpCodes::LoadValue { reg, value } => {
                write!(f, "LOADVALUE 0x{:02X} 0x{:02X}", reg, value)
            }
            OpCodes::Store { reg, addr } => write!(f, "STORE 0x{:02X} 0x{:02X}", reg, addr),
            OpCodes::Move {
                source_reg,
                target_reg,
            } => write!(f, "MOVE 0x{:02X} 0x{:02X}", source_reg, target_reg),
            OpCodes::AddInt {
                target_reg,
                reg1,
                reg2,
            } => write!(
                f,
                "ADDINT 0x{:02X} 0x{:02X} 0x{:02X}",
                target_reg, reg1, reg2
            ),
            OpCodes::AddFloat {
                target_reg,
                reg1,
                reg2,
            } => write!(
                f,
                "ADDFLOAT 0x{:02X} 0x{:02X} 0x{:02X}",
                target_reg, reg1, reg2
            ),
            OpCodes::Or {
                target_reg,
                reg1,
                reg2,
            } => write!(f, "OR 0x{:02X} 0x{:02X} 0x{:02X}", target_reg, reg1, reg2),
            OpCodes::And {
                target_reg,
                reg1,
                reg2,
            } => write!(f, "AND 0x{:02X} 0x{:02X} 0x{:02X}", target_reg, reg1, reg2),
            OpCodes::Xor {
                target_reg,
                reg1,
                reg2,
            } => write!(f, "XOR 0x{:02X} 0x{:02X} 0x{:02X}", target_reg, reg1, reg2),
            OpCodes::Rotate { reg, times } => write!(f, "ROTATE 0x{:02X} 0x{:02X}", reg, times),
            OpCodes::Jump { reg, addr } => write!(f, "JUMP 0x{:02X} 0x{:02X}", reg, addr),
            OpCodes::Halt => write!(f, "HALT"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn opcode_loadaddr_works() {
        let program = [0x14, 0xA3];
        let mut cpu = Cpu::init(&program);
        cpu.memory[0xA3] = 0xCD;
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x04], 0xCD)
    }

    #[test]
    pub fn opcode_loadvalue_works() {
        let program = [0x20, 0xA3];
        let mut cpu = Cpu::init(&program);
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x00], 0xA3)
    }

    #[test]
    pub fn opcode_store_works() {
        let program = [0x35, 0xB1];
        let mut cpu = Cpu::init(&program);
        cpu.registers[5] = 0x58;
        assert!(cpu.cycle());
        assert_eq!(cpu.memory[0xB1], 0x58)
    }

    #[test]
    pub fn opcode_move_works() {
        let program = [0x40, 0xA4];
        let mut cpu = Cpu::init(&program);
        cpu.registers[0xA] = 0xFF;
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x04], 0xFF)
    }

    #[test]
    pub fn opcode_addint_works() {
        let program = [0x57, 0x26];
        let mut cpu = Cpu::init(&program);
        cpu.registers[0x02] = 0x03;
        cpu.registers[0x06] = 0x05;
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x07], 0x08)
    }

    #[test]
    pub fn opcode_addfloat_works() {
        let program = [0x63, 0x4E];
        let mut cpu = Cpu::init(&program);
        cpu.registers[0x04] = 0x03;
        cpu.registers[0x0E] = 0x05;
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x03], 0x08)
    }

    #[test]
    pub fn opcode_or_works() {
        let program = [0x7C, 0xB4];
        let mut cpu = Cpu::init(&program);
        cpu.registers[0x0B] = 0xF0;
        cpu.registers[0x04] = 0x0F;
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x0C], 0xFF)
    }

    #[test]
    pub fn opcode_and_works() {
        let program = [0x80, 0x45];
        let mut cpu = Cpu::init(&program);
        cpu.registers[0x04] = 0xF3;
        cpu.registers[0x05] = 0x05;
        cpu.cycle();
        assert_eq!(cpu.registers[0x00], 0x01)
    }

    #[test]
    pub fn opcode_xor_works() {
        let program = [0x95, 0xF3];
        let mut cpu = Cpu::init(&program);
        cpu.registers[0x0F] = 0xF3;
        cpu.registers[0x03] = 0x05;
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x05], 0xF6)
    }

    #[test]
    pub fn opcode_rotate_works() {
        let program = [0xA4, 0x03];
        let mut cpu = Cpu::init(&program);
        cpu.registers[0x04] = 0x07;
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x04], 0xE0)
    }

    #[test]
    pub fn opcode_jump_works() {
        let program = [0xB4, 0x3C];
        let mut cpu = Cpu::init(&program);
        cpu.memory[0x3C] = 0xAB;
        cpu.registers[0x00] = 0x05;
        cpu.registers[0x04] = 0x05;
        assert!(cpu.cycle());
        assert_eq!(cpu.program_counter, 0xAB)
    }

    #[test]
    pub fn opcode_halt_works() {
        let program = [0xC0];
        let mut cpu = Cpu::init(&program);
        assert!(cpu.cycle());
        assert!(cpu.halted)
    }

    #[test]
    pub fn run_works_1() {
        let program = [0x14, 0x02, 0x34, 0x17, 0xC0, 0x00];
        let mut cpu = Cpu::init(&program);
        assert!(cpu.run());
        assert!(cpu.halted);
        assert_eq!(cpu.memory[0x17], 0x34)
    }

    #[test]
    pub fn run_works_2a() {
        let program = [0x13, 0xB8, 0xA3, 0x02, 0x33, 0xB8, 0xC0, 0x00, 0x0F];
        let mut cpu = Cpu::new();
        (0..program.len()).for_each(|a| {
            cpu.memory[0xB0 + a] = program[a];
        });
        cpu.program_counter = 0xB0;
        assert!(cpu.cycle());
        assert_eq!(cpu.registers[0x03], 0x0F);
        assert!(cpu.run());
        assert!(cpu.halted);
        assert_eq!(cpu.memory[0xB8], 0xC3);
    }

    #[test]
    pub fn cycle_with_illegal_instruction() {
        let program = [0xD3, 0x02];
        let mut cpu = Cpu::init(&program);
        assert!(!cpu.cycle());
        assert!(cpu.halted);
    }

    #[test]
    pub fn run_with_illegal_instruction() {
        let program = [0xD3, 0x02];
        let mut cpu = Cpu::init(&program);
        assert!(!cpu.run());
        assert!(cpu.halted);
    }
}
