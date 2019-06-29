extern crate failure;
extern crate termion;

mod entity;

use failure::Error;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, color,style};

use entity::Entity;

fn main() -> Result<(), Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    let screen_w = 80;
    let screen_h = 50;

    let mut player = Entity::new(screen_w / 2, screen_h / 2, '@', color::Rgb(0x0, 0x95, 0xff));
    let mut npc = Entity::new(screen_w / 2 - 5, screen_h / 2, '@', color::Rgb(0x0, 0x00, 0xff));

    writeln!(stdout, "{}{}", clear::All, cursor::Hide)?;

    writeln!(stdout, "{}", color::Fg(color::Rgb(0x0, 0x95, 0xff)))?;

    writeln!(stdout, "{}@", cursor::Goto(player.x as u16, player.y as u16))?;
    stdout.flush()?;

    for c in stdin.keys() {
        // clear
        writeln!(stdout, "{} ", cursor::Goto(player.x as u16, player.y as u16))?;
        // input
        match c? {
            Key::Char('h') => player.x -= 1,
            Key::Char('j') => player.y += 1,
            Key::Char('k') => player.y -= 1,
            Key::Char('l') => player.x += 1,
            Key::Char('q') => break,
            _ => {}
        }

        // render
        writeln!(stdout, "{}@", cursor::Goto(player.x as u16, player.y as u16))?;
        stdout.flush()?;
    }

    writeln!(stdout, "{}{}{}", style::Reset, cursor::Goto(1, 1), cursor::Show)?;

    Ok(())
}
