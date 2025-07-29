mod draw_line;

use crossterm::{cursor, execute, queue, style, terminal};
use std::io::{self, Write};

use crate::draw_line::draw_line;

const DRAW: [(u16, u16); 3] = [(0, 0), (8, 10), (5, 2)];

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )?;

    if DRAW.len() < 2 {
        return Ok(());
    }

    let mut from = DRAW[0];
    for point in &DRAW[1..] {
        let pixels = draw_line(from, *point)?;
        for px in pixels {
            queue!(stdout, cursor::MoveTo(px.0, px.1), style::Print('#'))?;
        }

        from = *point;
    }

    stdout.flush()?;

    execute!(stdout, cursor::MoveTo(0, 25), cursor::Show)?;
    println!();
    Ok(())
}
