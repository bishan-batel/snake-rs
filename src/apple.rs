use crate::snake::Snake;
use crossterm::{
    style::{self, Colorize},
    QueueableCommand,
};
use rand::prelude::*;
use std::io::Stdout;

pub struct Apple(u16, u16);

impl Apple {
    pub fn new(screen_size: &(u16, u16)) -> Self {
        Self(screen_size.0 / 2, screen_size.1 / 2)
    }

    pub fn tick(&mut self, screen_size: &(u16, u16), snake: &mut Snake) {
        let head = snake.get_head();
        if head.0 == self.0 && head.1 == self.1 {
            snake.grow();
            self.shuffle(screen_size);
        }

        self.0 = std::cmp::min(screen_size.0, std::cmp::max(0, self.0));
        self.1 = std::cmp::min(screen_size.1, std::cmp::max(0, self.1));
    }

    pub fn render(&self, stdout: &mut Stdout) -> crossterm::Result<()> {
        stdout
            .queue(crossterm::cursor::MoveTo(self.0, self.1))?
            .queue(style::PrintStyledContent("@".red()))?;
        Ok(())
    }

    fn shuffle(&mut self, screen_size: &(u16, u16)) {
        let mut rng = thread_rng();
        let pcent: (f32, f32) = (rng.gen(), rng.gen());
        self.0 = (pcent.0 * screen_size.0 as f32) as u16;
        self.1 = (pcent.1 * screen_size.1 as f32) as u16;
    }
}
