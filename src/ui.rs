use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders,Paragraph};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use termion::raw::IntoRawMode;


use crate::game::{Item,RenderMap, SnakeControl, FoodGroup};

pub struct UI {
    terminal: tui::Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>
}

pub struct Inputs {

}

impl UI {
    pub fn new() -> Result<Self, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        Ok(Self {
            terminal,
        })
    }

    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
    }

    fn init(&mut self) {
        unimplemented!();
    }

    pub fn get_snake_control(&self) -> SnakeControl {
        SnakeControl::None
    }

    async fn process_input() {

    }

    pub fn render(&mut self,map: RenderMap) {
        self.terminal.draw(|f| {
            let text :Vec<Spans> = map.iter().map(|row| {

                let row_spans : Vec<Span> = row.iter().map(|item| {
                    match item {
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