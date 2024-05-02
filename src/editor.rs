use std::io::{stdout, Write};

use anyhow::Ok;

use crossterm::{
    cursor,
    event::{self, read},
    style::{self, Color, Stylize},
    terminal::{self, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

use crate::buffer::Buffer;

enum Action {
    /// Enumeration of the actions that are currently supported in the
    /// Kostakis Editor.
    Quit,
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,

    EnterMode(Mode),
}

#[derive(Debug)]
enum Mode {
    READ,
    EDIT,
}

pub struct Editor {
    buffer: Buffer,
    stdout: std::io::Stdout,
    size: (u16, u16),
    cx: u16,
    cy: u16,
    mode: Mode,
}

impl Drop for Editor {
    fn drop(&mut self) {
        _ = self.stdout.flush().unwrap();
        _ = self.stdout.execute(LeaveAlternateScreen);
        _ = terminal::disable_raw_mode();
    }
}

impl Editor {
    pub fn new(buffer: Buffer) -> anyhow::Result<Self> {
        let mut stdout = stdout();

        terminal::enable_raw_mode()?;
        stdout.execute(terminal::EnterAlternateScreen)?;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        Ok(Editor {
            buffer,
            stdout,
            cx: 0,
            cy: 0,
            mode: Mode::READ,
            size: terminal::size()?,
        })
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.draw_buffer()?;
        self.draw_status_line()?;
        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn draw_buffer(&mut self) -> anyhow::Result<()> {
        for (i, line) in self.buffer.lines.iter().enumerate() {
            self.stdout.queue(cursor::MoveTo(0, i as u16))?;
            self.stdout.queue(style::Print(line))?;
        }

        Ok(())
    }

    pub fn draw_status_line(&mut self) -> anyhow::Result<()> {
        let mode = format!("{:?}", self.mode).to_uppercase();
        let position = format!("{}:{} ", self.cx, self.cy);
        let file = format!(
            " {}",
            self.buffer.file.as_deref().unwrap_or("No Name Found")
        );

        let file_width = self.size.0 - mode.len() as u16 - position.len() as u16;

        self.stdout.queue(cursor::MoveTo(0, self.size.1 - 2))?;
        self.stdout.queue(style::PrintStyledContent(
            mode.with(Color::Rgb { r: 0, g: 0, b: 0 })
                .on(Color::Rgb {
                    r: 184,
                    g: 144,
                    b: 243,
                })
                .bold(),
        ))?;

        self.stdout.queue(style::PrintStyledContent(
            format!("{:<width$}", file, width = file_width as usize)
                .with(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                })
                .bold()
                .on(Color::Rgb {
                    r: 67,
                    g: 70,
                    b: 89,
                }),
        ))?;
        self.stdout.queue(style::PrintStyledContent(
            position
                .with(Color::Rgb { r: 0, g: 0, b: 0 })
                .bold()
                .on(Color::Rgb {
                    r: 184,
                    g: 144,
                    b: 243,
                }),
        ))?;

        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.draw()?;
            if let Some(action) = self.handle_event(read()?)? {
                match action {
                    Action::Quit => break,
                    Action::MoveUp => {
                        self.cy = self.cy.saturating_sub(1);
                    }
                    Action::MoveDown => {
                        self.cy += 1u16;
                    }
                    Action::MoveLeft => {
                        self.cx = self.cx.saturating_sub(1);
                    }
                    Action::MoveRight => {
                        self.cx += 1u16;
                    }
                    Action::EnterMode(new_mode) => {
                        self.mode = new_mode;
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        if matches!(ev, event::Event::Resize(_, _)) {
            self.size = terminal::size()?;
        }

        match &self.mode {
            Mode::READ => self.handle_normal_event(ev),
            Mode::EDIT => self.handle_insert_event(ev),
        }
    }

    fn handle_normal_event(&self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        match ev {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Char('q') => Ok(Some(Action::Quit)),
                event::KeyCode::Up => Ok(Some(Action::MoveUp)),
                event::KeyCode::Down => Ok(Some(Action::MoveDown)),
                event::KeyCode::Left => Ok(Some(Action::MoveLeft)),
                event::KeyCode::Right => Ok(Some(Action::MoveRight)),
                event::KeyCode::Char('i') => Ok(Some(Action::EnterMode(Mode::EDIT))),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn handle_insert_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        match ev {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Esc => Ok(Some(Action::EnterMode(Mode::READ))),
                event::KeyCode::Char(c) => {
                    self.stdout.queue(style::Print(c))?;
                    self.cx += 1;
                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}
