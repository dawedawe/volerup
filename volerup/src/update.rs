use std::num::ParseIntError;

use crate::model::{Focus, Model};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use vole_rs::vole::Cpu;

pub(crate) enum Msg {
    /// Exit the application
    Exit,
    /// Load program into memory
    Load,
    Cycle,
    FocusNext,
    FocusPrevious,
    ScrollUp,
    ScrollDown,
    KeyInput {
        key: Event,
    },
}

pub(crate) fn handle_event(model: &mut Model) -> color_eyre::Result<Option<Msg>> {
    match event::read()? {
        // it's important to check KeyEventKind::Press to avoid handling key release events
        Event::Key(key) if key.kind == KeyEventKind::Press => Result::Ok(on_key_event(model, key)),
        _ => Result::Ok(None),
    }
}

fn on_key_event(model: &mut Model, key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Msg::Exit),
        KeyCode::Char('r') => Some(Msg::Load),
        KeyCode::Char(' ') => Some(Msg::Cycle),
        KeyCode::Tab => Some(Msg::FocusNext),
        KeyCode::BackTab => Some(Msg::FocusPrevious),
        _ if model.focus == Focus::Program => Some(Msg::KeyInput {
            key: crossterm::event::Event::Key(key),
        }),
        KeyCode::Up => Some(Msg::ScrollUp),
        KeyCode::Down => Some(Msg::ScrollDown),
        _ => None,
    }
}

pub(crate) fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Exit => {
            model.running = false;
        }
        Msg::Load => {
            let input = model
                .program_textarea
                .lines()
                .iter()
                .map(|s| {
                    let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
                    u8::from_str_radix(s, 16)
                })
                .collect::<Vec<Result<u8, ParseIntError>>>();
            if input.iter().any(|r| r.is_err()) {
                model.error_msg = Some("only byte values in hex notation allowed")
            } else {
                model.error_msg = None;
                let input = input
                    .into_iter()
                    .map(|r| r.expect("expected a parsed u8"))
                    .collect::<Vec<u8>>();
                model.cpu = Cpu::init(&input);
            }
        }
        Msg::Cycle => {
            if !model.cpu.halted {
                model.cpu.cycle();
            }
        }
        Msg::FocusNext => match model.focus {
            Focus::Registers => model.focus = Focus::Memory,
            Focus::Memory => model.focus = Focus::Program,
            Focus::Program => model.focus = Focus::Registers,
        },
        Msg::FocusPrevious => match model.focus {
            Focus::Registers => model.focus = Focus::Program,
            Focus::Memory => model.focus = Focus::Registers,
            Focus::Program => model.focus = Focus::Memory,
        },
        Msg::ScrollUp => match model.focus {
            Focus::Registers => model.registers_scroll = model.registers_scroll.saturating_sub(1),
            Focus::Memory => model.memory_scroll = model.memory_scroll.saturating_sub(1),
            _ => (),
        },
        Msg::ScrollDown => match model.focus {
            Focus::Registers if model.registers_scroll < model.cpu.registers.len() => {
                model.registers_scroll = model.registers_scroll.saturating_add(1)
            }
            Focus::Memory if model.memory_scroll < model.cpu.memory.len() => {
                model.memory_scroll = model.memory_scroll.saturating_add(1)
            }
            _ => (),
        },
        Msg::KeyInput { key } if model.focus == Focus::Program => {
            model.program_textarea.input(key);
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::{Msg, update};
    use crate::model::Model;

    #[test]
    fn test_exit_msg() {
        let mut model = Model::default();
        update(&mut model, Msg::Exit);
        assert!(!model.running)
    }
}
