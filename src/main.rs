mod shapes;

use std::io::{Write, stdout};

use crossterm::{
    cursor::{self, MoveTo},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind,
    },
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

type Point = (u16, u16, char);

fn display_message(message: &str) -> std::io::Result<()> {
    let (_cols, rows) = size()?;
    let mut out = stdout();

    execute!(out, MoveTo(0, rows - 1))?;
    execute!(out, Clear(ClearType::CurrentLine))?;

    write!(out, "{}", message)?;
    out.flush()?;
    Ok(())
}

macro_rules! status {
    ($($arg:tt)*) => {
        display_message(&format!($($arg)*))?
    };
}

fn draw(x: u16, y: u16, ch: char) -> std::io::Result<()> {
    execute!(
        stdout(),
        MoveTo(x, y),
        SetForegroundColor(Color::White),
        Print(ch),
        ResetColor
    )?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::Hide,
        EnableMouseCapture
    )?;

    let mut points: Vec<Point> = vec![];

    let mut character: char = '*';
    let mut is_select_character: bool = false;

    loop {
        stdout.flush()?;
        if event::poll(std::time::Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(k) => {
                    if is_select_character {
                        if let KeyCode::Char(c) = k.code {
                            character = c;
                            is_select_character = false;
                            status!("Character set to '{}'", character);
                        }
                        continue;
                    }

                    match k.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('k') => execute!(stdout, Clear(ClearType::All))?,
                        KeyCode::Char('a') => {
                            is_select_character = true;
                            status!("Press the new drawing character:");
                        }
                        _ => {}
                    }
                }

                Event::Mouse(me) => match me.kind {
                    MouseEventKind::Down(MouseButton::Left)
                    | MouseEventKind::Drag(MouseButton::Left) => {
                        let x = me.column;
                        let y = me.row;

                        points.push((x, y, character));

                        let (_cols, rows) = size()?;
                        if y + 1 < rows {
                            draw(x, y, character)?;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout, cursor::Show, DisableMouseCapture)?;
    Ok(())
}
