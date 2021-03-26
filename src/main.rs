use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{self, Colorize},
    terminal, QueueableCommand, Result,
};
use snake::Direction;
use std::io::{stdout, Stdout, Write};
use std::time;

// Modules
mod apple;
mod snake;

fn main() -> Result<()> {
    let mut stdout: Stdout = stdout();
    let msg = "Game Over, Play Again [y/n]: ";
    let mut quit: bool;

    let game_over = &mut (|stdout: &mut Stdout| -> Result<()> {
        let term_size = terminal::size().unwrap_or((0, 0));
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(
                (term_size.0 - msg.len() as u16) / 2,
                term_size.1 / 2 - 3,
            ))?
            .queue(style::PrintStyledContent(msg.red()))?;
        stdout.flush()?;
        Ok(())
    });

    loop {
        game(&mut stdout)?;

        game_over(&mut stdout)?;

        loop {
            let line = read_line().unwrap_or(String::from(""));
            let starts_with_no = line.starts_with('n');

            if !(line.starts_with('y') || starts_with_no) {
                game_over(&mut stdout)?;
                continue;
            }
            quit = starts_with_no;
            break;
        }
        if quit {
            stdout
                .queue(terminal::Clear(terminal::ClearType::All))?
                .queue(cursor::MoveTo(0, 0))?;
            break;
        }
    }
    Ok(())
}

fn game(stdout: &mut Stdout) -> Result<()> {
    let poll_duration = time::Duration::from_millis(1);

    // game objects
    let mut screen_size = terminal::size()?;
    let mut snake = snake::Snake::new(&screen_size);
    let mut apple = apple::Apple::new(&screen_size);

    terminal::enable_raw_mode()?;
    stdout.queue(cursor::Hide)?;

    let mut prev_time = time::SystemTime::now();

    // main game loop
    loop {
        // update screen size
        screen_size = terminal::size()?;

        // clear

        if prev_time.elapsed().unwrap().as_millis() > 100 {
            prev_time = time::SystemTime::now();

            // update game objects
            if snake.tick(&screen_size) {
                break;
            }
            apple.tick(&screen_size, &mut snake);
        }

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        apple.render(stdout)?;
        snake.render(stdout)?;
        stdout.flush()?;

        // input handling
        if event::poll(poll_duration)? {
            match event::read()? {
                Event::Key(key) => {
                    match key.code {
                        // Direction movement
                        KeyCode::Char('w') => snake.change_dir(Direction::Up),
                        KeyCode::Char('a') => snake.change_dir(Direction::Left),
                        KeyCode::Char('s') => snake.change_dir(Direction::Down),
                        KeyCode::Char('d') => snake.change_dir(Direction::Right),
                        // Queues program quit
                        KeyCode::Char('q') => break,
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
    }

    // finalize screen to clean up
    terminal::disable_raw_mode()?;
    stdout
        .queue(cursor::Show)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?;
    Ok(())
}

pub fn read_line() -> Result<String> {
    let mut line = String::new();
    while let Event::Key(event::KeyEvent { code, .. }) = event::read()? {
        match code {
            KeyCode::Enter => {
                break;
            }
            KeyCode::Char(c) => {
                line.push(c);
            }
            _ => {}
        }
    }

    Ok(line)
}
