use crate::model::Model;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub enum Msg {
    Exit,
    Cycle,
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
