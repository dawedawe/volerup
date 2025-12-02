use vole_rs::vole::Cpu;

#[derive(Debug, PartialEq)]
pub(crate) enum Focus {
    Registers,
    Memory,
    Program,
}

#[derive(Debug)]
pub(crate) struct Model {
    pub(crate) cpu: vole_rs::vole::Cpu,
    pub program: Vec<u8>,
    pub running: bool,
    pub focus: Focus,
    pub memory_scroll: usize,
    pub program_scroll: usize,
    pub registers_scroll: usize,
}

impl Default for Model {
    fn default() -> Self {
        let program = vec![0x14, 0x02, 0x34, 0x17, 0xC0, 0x00];
        Model {
            cpu: Cpu::init(&program),
            program,
            running: true,
            focus: Focus::Memory,
            memory_scroll: 0,
            program_scroll: 0,
            registers_scroll: 0,
        }
    }
}
