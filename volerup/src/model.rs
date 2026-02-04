use ratatui::style::{Color, Style};
use tui_textarea::TextArea;
use vole_rs::vole::Cpu;

use crate::update::parse_program_text;

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
    pub(crate) modified_register: Option<usize>,
    pub(crate) modified_memory: Option<usize>,
    pub(crate) show_help: bool,
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
        let program_text = program
            .chunks(2)
            .map(|h| {
                if h.len() == 2 {
                    format!("0x{:02X}{:02X}", h[0], h[1])
                } else {
                    format!("0x{:02X}", h[0])
                }
            })
            .collect();
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
            modified_register: None,
            modified_memory: None,
            show_help: false,
            error_msg: None,
        }
    }

    pub(crate) fn init_from_source(program_text: &str) -> Result<Model<'a>, String> {
        let lines = program_text
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let program = parse_program_text(&lines);
        let mut program_textarea = TextArea::new(lines);
        let style = Style::default().fg(Color::Green);
        program_textarea.set_line_number_style(style);
        program_textarea.set_style(style);
        match program {
            Ok(program) => {
                let model = Model {
                    cpu: Cpu::init(&program),
                    program_textarea,
                    running: true,
                    focus: Focus::Memory,
                    memory_scroll: 0,
                    registers_scroll: 0,
                    modified_register: None,
                    modified_memory: None,
                    show_help: false,
                    error_msg: None,
                };
                Ok(model)
            }
            Err(e) => Err(e.into()),
        }
    }
}
