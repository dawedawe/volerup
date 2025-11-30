use crate::model::{Focus, Model};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub enum Msg {
    Exit,
    Cycle,
    FocusNext,
    FocusPrevious,
    ScrollUp,
    ScrollDown,
}

pub fn handle_event(model: &mut Model) -> color_eyre::Result<Option<Msg>> {
    match event::read()? {
        // it's important to check KeyEventKind::Press to avoid handling key release events
        Event::Key(key) if key.kind == KeyEventKind::Press => Result::Ok(on_key_event(model, key)),
        _ => Result::Ok(None),
    }
}

fn on_key_event(_model: &mut Model, key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Msg::Exit),
        KeyCode::Char(' ') => Some(Msg::Cycle),
        KeyCode::Tab => Some(Msg::FocusNext),
        KeyCode::BackTab => Some(Msg::FocusPrevious),
        KeyCode::Up => Some(Msg::ScrollUp),
        KeyCode::Down => Some(Msg::ScrollDown),
        _ => None,
    }
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Exit => {
            model.running = false;
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
            Focus::Program => model.program_scroll = model.program_scroll.saturating_sub(1),
        },
        Msg::ScrollDown => match model.focus {
            Focus::Registers if model.registers_scroll < model.cpu.registers.len() => {
                model.registers_scroll = model.registers_scroll.saturating_add(1)
            }
            Focus::Memory if model.memory_scroll < model.cpu.memory.len() => {
                model.memory_scroll = model.memory_scroll.saturating_add(1)
            }
            Focus::Program if model.program_scroll < model.program.len() => {
                model.program_scroll = model.program_scroll.saturating_add(1)
            }
            _ => (),
        },
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
