use std::io;
// use termion::input::TermRead;
use crossbeam::channel::{bounded,Receiver,TrySendError};
use std::thread;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders,Paragraph};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use termion::raw::IntoRawMode;
// use std::future::Future;

use termion::event::Key;
use termion::input::TermRead;

use crate::game::{Item,RenderMap, SnakeControl, FoodGroup};

#[derive(PartialEq)]
pub enum UIControl {
    SnakeControl(SnakeControl),
    ExitProgram
}

impl UIControl {
    pub fn get_snake_control(self) -> SnakeControl {
        match self {
            UIControl::SnakeControl(c) => c,
            _ => SnakeControl::None,
        }
    }
}

pub struct UI {
    terminal: tui::Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
    // input_thread_handle: std::thread::JoinHandle<()>,
    receiver_channel: Receiver<Input>,
}

pub struct Input {
    k: Key,
}

impl UI {
    pub fn new() -> Result<Self, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let (sender,receiver_channel) = bounded::<Input>(1);

        let _input_thread_handle = {
            let tx = sender.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        // if we have a key, then try to send it
                        if let Err(err) = tx.try_send(Input{k:key}) {
                            if let TrySendError::Disconnected(_) = err {
                                return;
                            }
                        }
                    }
                }
            })
        };

        Ok(Self {
            terminal,
            // input_thread_handle,
            receiver_channel,
        })
    }

    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
    }

    pub fn get_control(&self) -> UIControl {
        if let Ok(input) = self.receiver_channel.try_recv() {
            match input.k {
                Key::Left  => UIControl::SnakeControl(SnakeControl::Left),
                Key::Right => UIControl::SnakeControl(SnakeControl::Right),
                Key::Up    => UIControl::SnakeControl(SnakeControl::Up),
                Key::Down  => UIControl::SnakeControl(SnakeControl::Down),
                Key::Esc | Key::Ctrl('c') | Key::Char('q') 
                           => UIControl::ExitProgram,
                _          => UIControl::SnakeControl(SnakeControl::None),
            }
        } else {
            UIControl::SnakeControl(SnakeControl::None)
        }
    }

    pub fn render(&mut self,map: RenderMap) {
        self.terminal.draw(|f| {

            let height = map[0].len();
            let width = map.len();

            let text :Vec<Spans> = (0..height).rev().map(|y| {
                let row_spans : Vec<Span> = (0..width).map(|x| {
                    match map[x][y] {
                        Item::Food(food_type) => {
                            match food_type {
                                FoodGroup::Grow   => Span::styled(" o", Style::default().fg(Color::Blue)),
                                FoodGroup::Poison => Span::styled(" o", Style::default().fg(Color::Red)),
                                FoodGroup::Shrink => Span::styled(" o", Style::default().fg(Color::Yellow)),
                            }
                        }
                        Item::SnakeHead => Span::styled(" O", Style::default().fg(Color::Green)),
                        Item::Snake     => Span::styled(" *", Style::default().fg(Color::LightGreen)),
                        Item::Nothing   => Span::styled(" -", Style::default().fg(Color::Gray)),
                    }

                }).collect();
                return Spans::from(row_spans);

            }).collect();   
            let size = f.size();
            let block = Block::default()
                .title("Snek")
                .borders(Borders::ALL);
            let paragraph = Paragraph::new(text).block(block);
            f.render_widget(paragraph, size);
        }).unwrap();
    }
}