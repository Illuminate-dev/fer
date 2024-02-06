use crate::app::App;
use crate::app::Mode;
use anyhow::Result;
use termion::event::Key;

pub enum ReturnCommand {
    Quit,
    AddChar(char),
    DelChar,
    SaveFile,
    None,
}

pub struct InputHandler {}

impl InputHandler {
    pub fn handle_input(app: &mut App, key: Key) -> Result<ReturnCommand> {
        match key {
            Key::Ctrl('q') => Ok(ReturnCommand::Quit),
            Key::Ctrl('s') => Ok(ReturnCommand::SaveFile),
            _ => match app.current_mode {
                Mode::Normal => Self::handle_normal(app, key),
                Mode::Insert => Self::handle_insert(app, key),
            },
        }
    }

    fn handle_normal(app: &mut App, key: Key) -> Result<ReturnCommand> {
        match key {
            Key::Char('i') => app.update_mode(Mode::Insert)?,
            Key::Char('h') => app.move_cursor(-1, 0)?,
            Key::Char('j') => app.move_cursor(0, 1)?,
            Key::Char('k') => app.move_cursor(0, -1)?,
            Key::Char('l') => app.move_cursor(1, 0)?,
            _ => {}
        }
        Ok(ReturnCommand::None)
    }
    fn handle_insert(app: &mut App, key: Key) -> Result<ReturnCommand> {
        match key {
            Key::Esc => app.update_mode(Mode::Normal)?,
            Key::Char(c) => return Ok(ReturnCommand::AddChar(c)),
            Key::Backspace => return Ok(ReturnCommand::DelChar),
            _ => {}
        }
        Ok(ReturnCommand::None)
    }
}
