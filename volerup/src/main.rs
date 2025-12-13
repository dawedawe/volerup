pub mod model;
pub mod update;
pub mod view;

use std::env;

use model::Model;
use update::{handle_event, update};
use view::view;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    match get_model(args) {
        Ok(mut model) => {
            let mut terminal = ratatui::init();

            while model.running {
                terminal.draw(|f| view(&model, f))?;
                if let Some(msg) = handle_event(&mut model)? {
                    update(&mut model, msg)
                }
            }
        }
        Err(e) => eprintln!("{e}"),
    }

    ratatui::restore();
    color_eyre::Result::Ok(())
}

fn get_model<'a>(args: Vec<String>) -> Result<Model<'a>, String> {
    if args.len() == 1 {
        Ok(Model::default())
    } else if args.len() == 2 {
        match std::fs::read_to_string(args[1].as_str()) {
            Ok(input) => Model::init_from_source(input.as_str()),
            Err(e) => {
                let s = e.to_string();
                Err(s)
            }
        }
    } else {
        let usage = format!("Usage: {} [path_to_file]", args[0]);
        Err(usage)
    }
}

#[cfg(test)]
mod tests {
    use crate::get_model;

    #[test]
    fn test_bad_args() {
        let model = get_model(vec!["foo".into(), "bar".into()]);
        assert!(model.is_err())
    }

    #[test]
    fn test_no_args() {
        let model = get_model(vec!["volerup".into()]);
        assert!(model.is_ok())
    }
}
