use crossterm::{
    cursor,
    style::{self, Colorize},
    terminal, QueueableCommand, Result,
};
use std::io::Stdout;
use std::vec;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_opposite(&self) -> Self {
        match *self {
            Self::Up => return Self::Down,
            Self::Down => return Self::Up,
            Self::Right => return Self::Left,
            Self::Left => return Self::Right,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Segment(pub u16, pub u16);

impl Segment {
    fn slither(&self, screen_size: &(u16, u16), dir: &Direction) -> Self {
        let mut segment = self.clone();
        match dir {
            Direction::Up => segment.1 -= 1,
            Direction::Right => segment.0 += 1,
            Direction::Left => segment.0 -= 1,
            Direction::Down => segment.1 += 1,
        }
        if segment.0 <= 0 {
            segment.0 = screen_size.0 - 2;
        } else if segment.0 >= screen_size.0 - 1 {
            segment.0 = 1;
        }
        if segment.1 <= 0 {
            segment.1 = screen_size.1 - 2;
        } else if segment.1 >= screen_size.1 - 1 {
            segment.1 = 1;
        }
        segment
    }
}

pub struct Snake {
    segments: std::vec::Vec<Segment>,
    dir: Direction,
}

impl Snake {
    pub fn new(screen_size: &(u16, u16)) -> Self {
        Self {
            segments: vec![Segment(screen_size.0 / 2, screen_size.1 / 2)],
            dir: Direction::Up,
        }
    }

    pub fn tick(&mut self, screen_size: &(u16, u16)) -> bool {
        // pushes all segments foreward
        self.segments
            .insert(0, self.get_head().slither(screen_size, &self.dir));
        self.segments.pop();

        // If head collides with any segment, game over
        let (head, body) = self.segments.split_first().unwrap_or((&Segment(0, 0), &[]));
        for segment in body {
            if *segment == *head {
                return true;
            }
        }
        false
    }

    pub fn render(&self, stdout: &mut Stdout) -> Result<()> {
        let (head, body) = self.segments.split_first().unwrap();
        for segment in body {
            stdout
                .queue(cursor::MoveTo(segment.0, segment.1))?
                .queue(style::PrintStyledContent("#".green()))?;
        }
        stdout
            .queue(cursor::MoveTo(head.0, head.1))?
            .queue(style::PrintStyledContent("@".green()))?;
        Ok(())
    }

    pub fn grow(&mut self) {
        self.segments.push(self.get_head().clone());
    }

    pub fn get_head(&self) -> &Segment {
        &self.segments[0]
    }

    pub fn change_dir(&mut self, dir: Direction) {
        if self.dir.get_opposite() != dir {
            self.dir = dir;
        }
    }
}
