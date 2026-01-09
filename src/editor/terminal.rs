use core::fmt::Display;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::io::{Error, Write, stdout};

pub struct Size {
    pub height: usize,
    pub width: usize,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
    pub fn print(string: impl Display) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }
    fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        let width = width as usize;
        let height = height as usize;
        Ok(Size { height, width })
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
