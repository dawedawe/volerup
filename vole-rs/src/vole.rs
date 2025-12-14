use std::fmt::Display;

use crate::floating::Floating;

/// Represents the state of the Vole-speaking CPU.
#[derive(Debug)]
pub struct Cpu {
    /// The general purpose registers.
    pub registers: [u8; 16],
    /// The main memory.
    pub memory: [u8; 256],
    /// Points to the next instruction in memory to fetch.
    pub program_counter: usize,
    /// Holds the next instruction to decode and execute.
    pub instruction_register: u16,
    /// Counts how many cycles have been processed.
    pub cycle: u32,
    /// True if the [Cpu] has halted, false otherwise.
    pub halted: bool,
}

impl Cpu {
    /// Creates a new [Cpu].
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            memory: [0; 256],
            program_counter: 0,
            instruction_register: 0x0000,
            cycle: 0,
            halted: false,
        }
    }

    /// Initializes a new [Cpu] with the given program loaded into memory.
    pub fn init(program: &[u8]) -> Self {
        let mut cpu = Cpu::new();
        if program.len() > cpu.memory.len() {
            panic!("given program does not fit into memory");
        }
        cpu.memory[..program.len()].copy_from_slice(program);
        cpu
    }

    /// Depending on the [Cpu::program_counter], fetches the next instruction from memory.
    pub fn fetch(&mut self) {
        let instr_byte0 = self.memory[self.program_counter];
        let instr_byte1 = self.memory[self.program_counter + 1];
        let mut instr: u16 = (instr_byte0 as u16) << 8;
        instr |= instr_byte1 as u16;
        self.instruction_register = instr;
    }

    /// Get the bits representing the [OpCode].
    pub fn get_opcode_bits(instr: Instruction) -> u8 {
        (instr >> 12) as u8
    }

    /// Get the bits representing the first operand.
    pub fn get_operand1_bits(instr: Instruction) -> u8 {
        ((instr & 0x0F00) >> 8) as u8
    }

    /// Get the bits representing the second operand.
    pub fn get_operand2_bits(instr: Instruction) -> u8 {
        ((instr & 0x00F0) >> 4) as u8
    }

    /// Get the bits representing the third operand.
    pub fn get_operand3_bits(instr: Instruction) -> u8 {
        (instr & 0x000F) as u8
    }

    /// Get the bits representing the second and third operand.
    pub fn get_operand23_bits(instr: Instruction) -> u8 {
        (instr & 0x00FF) as u8
    }

    /// Get the bits representing the all operands.
    pub fn get_operand_bits(instr: Instruction) -> u16 {
        instr & 0x0FFF
    }

    /// Decode the bits in the [Cpu::instruction_register] into an [OpCode].
    pub fn decode(&self) -> Option<OpCode> {
        let opcode_bits = Cpu::get_opcode_bits(self.instruction_register);
        let operand1 = Cpu::get_operand1_bits(self.instruction_register);
        let operand2 = Cpu::get_operand2_bits(self.instruction_register);
        let operand3 = Cpu::get_operand3_bits(self.instruction_register);
        match opcode_bits {
            0x1 => Some(OpCode::LoadAddr {
                reg: operand1,
                addr: Cpu::get_operand23_bits(self.instruction_register),
            }),
            0x2 => Some(OpCode::LoadValue {
                reg: operand1,
                value: Cpu::get_operand23_bits(self.instruction_register),
            }),
            0x3 => Some(OpCode::Store {
                reg: operand1,
                addr: Cpu::get_operand23_bits(self.instruction_register),
            }),
            0x4 => Some(OpCode::Move {
                source_reg: operand2,
                target_reg: operand3,
            }),
            0x5 => Some(OpCode::AddInt {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0x6 => Some(OpCode::AddFloat {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0x7 => Some(OpCode::Or {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0x8 => Some(OpCode::And {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0x9 => Some(OpCode::Xor {
                target_reg: operand1,
                reg1: operand2,
                reg2: operand3,
            }),
            0xA => Some(OpCode::Rotate {
                reg: operand1,
                times: operand3,
            }),
            0xB => Some(OpCode::Jump {
                reg: operand1,
                addr: Cpu::get_operand23_bits(self.instruction_register),
            }),
            0xC => Some(OpCode::Halt),
            _ => None,
        }
    }

    /// Execute the given [OpCode].
    pub fn execute(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::LoadAddr { reg, addr: address } => {
                self.registers[reg as usize] = self.memory[address as usize];
            }
            OpCode::LoadValue { reg, value } => {
                self.registers[reg as usize] = value;
            }
            OpCode::Store { reg, addr } => {
                self.memory[addr as usize] = self.registers[reg as usize];
            }
            OpCode::Move {
                source_reg,
                target_reg,
            } => {
                self.registers[target_reg as usize] = self.registers[source_reg as usize];
            }
            OpCode::AddInt {
                target_reg,
                reg1,
                reg2,
            } => {
                self.registers[target_reg as usize] =
                    self.registers[reg1 as usize] + self.registers[reg2 as usize];
            }
            OpCode::AddFloat {
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
            OpCode::Or {
                target_reg,
                reg1,
                reg2,
            } => {
                self.registers[target_reg as usize] =
                    self.registers[reg1 as usize] | self.registers[reg2 as usize];
            }
            OpCode::And {
                target_reg,
                reg1,
                reg2,
            } => {
                self.registers[target_reg as usize] =
                    self.registers[reg1 as usize] & self.registers[reg2 as usize];
            }
            OpCode::Xor {
                target_reg,
                reg1,
                reg2,
            } => {
                self.registers[target_reg as usize] =
                    self.registers[reg1 as usize] ^ self.registers[reg2 as usize];
            }
            OpCode::Rotate { reg, times } => {
                self.registers[reg as usize] =
                    self.registers[reg as usize].rotate_right(times as u32)
            }
            OpCode::Jump { reg, addr } => {
                if self.registers[0] == self.registers[reg as usize] {
                    self.program_counter = self.memory[addr as usize] as usize;
                }
            }
            OpCode::Halt => {
                self.halted = true;
            }
        }

        match opcode {
            OpCode::Jump { reg: _, addr: _ } => (),
            _ => self.program_counter += 2,
        }
    }

    /// Do a full fetch-decode-ececute cycle.
    /// Returns false if instruction was illegal, true otherwise.
    pub fn cycle(&mut self) -> bool {
        self.fetch();
        if let Some(opcode) = self.decode() {
            self.execute(opcode);
            self.cycle = self.cycle.wrapping_add(1);
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

/// Type representing the 16 bit instructions of Vole.
pub type Instruction = u16;

/// The Vole opcodes.
pub enum OpCode {
    /// 0x1RXY - LOAD memory cell XY into register R.
    LoadAddr { reg: u8, addr: u8 },
    /// 0x2RXY - LOAD value XY into register R.
    LoadValue { reg: u8, value: u8 },
    /// 0x3RXY - STORE value in register R in memory cell XY.
    Store { reg: u8, addr: u8 },
    /// 0x40RS - MOVE register R to register S.
    Move { source_reg: u8, target_reg: u8 },
    /// 0x5RST - ADD registers R and S as integers, store the result in register T.
    AddInt { target_reg: u8, reg1: u8, reg2: u8 },
    /// 0x6RST - ADD registers R and S as floats, store the result in register T.
    AddFloat { target_reg: u8, reg1: u8, reg2: u8 },
    /// 0x7RST - OR registers R and S, store the result in register T.
    Or { target_reg: u8, reg1: u8, reg2: u8 },
    /// 0x8RST - AND registers R and S, store the result in register T.
    And { target_reg: u8, reg1: u8, reg2: u8 },
    /// 0x9RST - XOR registers R and S, store the result in register T.
    Xor { target_reg: u8, reg1: u8, reg2: u8 },
    /// 0xAR0X - ROTATE register R X times to the right.
    Rotate { reg: u8, times: u8 },
    /// 0xBRXY - JUMP to instruction at memory cell XY if register R equals register 0.
    Jump { reg: u8, addr: u8 },
    /// "0xC000 - HALT the execution.
    Halt,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::LoadAddr { reg, addr } => write!(f, "LOADADDR 0x{:02X} 0x{:02X}", reg, addr),
            OpCode::LoadValue { reg, value } => {
                write!(f, "LOADVALUE 0x{:02X} 0x{:02X}", reg, value)
            }
            OpCode::Store { reg, addr } => write!(f, "STORE 0x{:02X} 0x{:02X}", reg, addr),
            OpCode::Move {
                source_reg,
                target_reg,
            } => write!(f, "MOVE 0x{:02X} 0x{:02X}", source_reg, target_reg),
            OpCode::AddInt {
                target_reg,
                reg1,
                reg2,
            } => write!(
                f,
                "ADDINT 0x{:02X} 0x{:02X} 0x{:02X}",
                target_reg, reg1, reg2
            ),
            OpCode::AddFloat {
                target_reg,
                reg1,
                reg2,
            } => write!(
                f,
                "ADDFLOAT 0x{:02X} 0x{:02X} 0x{:02X}",
                target_reg, reg1, reg2
            ),
            OpCode::Or {
                target_reg,
                reg1,
                reg2,
            } => write!(f, "OR 0x{:02X} 0x{:02X} 0x{:02X}", target_reg, reg1, reg2),
            OpCode::And {
                target_reg,
                reg1,
                reg2,
            } => write!(f, "AND 0x{:02X} 0x{:02X} 0x{:02X}", target_reg, reg1, reg2),
            OpCode::Xor {
                target_reg,
                reg1,
                reg2,
            } => write!(f, "XOR 0x{:02X} 0x{:02X} 0x{:02X}", target_reg, reg1, reg2),
            OpCode::Rotate { reg, times } => write!(f, "ROTATE 0x{:02X} 0x{:02X}", reg, times),
            OpCode::Jump { reg, addr } => write!(f, "JUMP 0x{:02X} 0x{:02X}", reg, addr),
            OpCode::Halt => write!(f, "HALT"),
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
        assert_eq!(cpu.cycle, 0);
        assert!(cpu.cycle());
        assert_eq!(cpu.cycle, 1);
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
        assert_eq!(cpu.memory[0x17], 0x34);
        assert_eq!(cpu.cycle, 3);
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
        assert_eq!(cpu.cycle, 0);
    }

    #[test]
    pub fn run_with_illegal_instruction() {
        let program = [0xD3, 0x02];
        let mut cpu = Cpu::init(&program);
        assert!(!cpu.run());
        assert!(cpu.halted);
        assert_eq!(cpu.cycle, 0);
    }
}
