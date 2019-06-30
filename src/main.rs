extern crate failure;
extern crate termion;

mod entity;

use failure::Error;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

use entity::Entity;

fn render_entity<W: Write>(stdout: &mut W, e: &Entity) {
    writeln!(
        stdout,
        "{}{}@",
        cursor::Goto(e.x as u16, e.y as u16),
        color::Fg(e.color)
    ).unwrap();
}

fn clear_entity<W: Write>(stdout: &mut W, e: &Entity) {
    writeln!(
        stdout,
        "{} ",
        cursor::Goto(e.x as u16, e.y as u16),
    ).unwrap();
}

fn main() -> Result<(), Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    let screen_w = 80;
    let screen_h = 50;

    let mut player = Entity::new(screen_w / 2, screen_h / 2, '@', color::Rgb(0x0, 0x95, 0xff));
    let npc = Entity::new(
        screen_w / 2 - 5,
        screen_h / 2,
        '@',
        color::Rgb(0xff, 0xff, 0x00),
    );

    writeln!(stdout, "{}{}", clear::All, cursor::Hide)?;

    render_entity(&mut stdout, &npc);
    render_entity(&mut stdout, &player);
    stdout.flush()?;

    for c in stdin.keys() {
        // clear
        clear_entity(&mut stdout, &player);
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
        render_entity(&mut stdout, &npc);
        render_entity(&mut stdout, &player);
        stdout.flush()?;
    }

    writeln!(
        stdout,
        "{}{}{}",
        style::Reset,
        cursor::Goto(1, 1),
        cursor::Show
    )?;

    Ok(())
}
