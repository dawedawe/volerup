use vole_rs::vole::Cpu;

#[derive(Debug)]
pub struct Model {
    pub cpu: vole_rs::vole::Cpu,
    pub running: bool,
}

impl Default for Model {
    fn default() -> Self {
        let program = [0x14, 0x02, 0x34, 0x17, 0xC0, 0x00];
        Model {
            running: true,
            cpu: Cpu::init(&program),
        }
    }
}
