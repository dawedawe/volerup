use ratatui::style::{Color, Style};
use tui_textarea::TextArea;
use vole_rs::vole::Cpu;

#[derive(Debug, PartialEq)]
pub(crate) enum Focus {
    Registers,
    Memory,
    Program,
}

#[derive(Debug)]
pub(crate) struct Model<'a> {
    pub(crate) cpu: vole_rs::vole::Cpu,
    pub(crate) program_textarea: TextArea<'a>,
    pub(crate) running: bool,
    pub(crate) focus: Focus,
    pub(crate) memory_scroll: usize,
    pub(crate) registers_scroll: usize,
    pub(crate) error_msg: Option<&'a str>,
}

impl<'a> Default for Model<'a> {
    fn default() -> Self {
        let program = vec![0x14, 0x02, 0x34, 0x17, 0xC0, 0x00];
        Model::init(program)
    }
}

impl<'a> Model<'a> {
    pub(crate) fn init(program: Vec<u8>) -> Self {
        let program_text = program.iter().map(|h| format!("0x{:02X}", h)).collect();
        let mut program_textarea = TextArea::new(program_text);
        let style = Style::default().fg(Color::Green);
        program_textarea.set_line_number_style(style);
        program_textarea.set_style(style);

        Model {
            cpu: Cpu::init(&program),
            program_textarea,
            running: true,
            focus: Focus::Memory,
            memory_scroll: 0,
            registers_scroll: 0,
            error_msg: None,
        }
    }
}
