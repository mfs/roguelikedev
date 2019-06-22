extern crate failure;
extern crate termion;

use failure::Error;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, color,style};

fn main() -> Result<(), Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    let screen_w = 80;
    let screen_h = 50;

    let mut player_x = screen_w / 2;
    let mut player_y = screen_h / 2;

    writeln!(stdout, "{}{}", clear::All, cursor::Hide)?;

    writeln!(stdout, "{}", color::Fg(color::Rgb(0x0, 0x95, 0xff)))?;

    writeln!(stdout, "{}@", cursor::Goto(player_x, player_y))?;
    stdout.flush()?;

    for c in stdin.keys() {
        // clear
        writeln!(stdout, "{} ", cursor::Goto(player_x, player_y))?;
        // input
        match c? {
            Key::Char('h') => player_x -= 1,
            Key::Char('j') => player_y += 1,
            Key::Char('k') => player_y -= 1,
            Key::Char('l') => player_x += 1,
            Key::Char('q') => break,
            _ => {}
        }

        // render
        writeln!(stdout, "{}@", cursor::Goto(player_x, player_y))?;
        stdout.flush()?;
    }

    writeln!(stdout, "{}{}{}", style::Reset, cursor::Goto(1, 1), cursor::Show)?;

    Ok(())
}
