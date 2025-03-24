mod game;
mod renderer;
use crate::renderer::render_gameover_menu;
use crate::renderer::render_start_menu;
use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use ratatui::backend::CrosstermBackend;
use renderer::render;
use std::io::{self, stdout};
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut game = Game::new();
    let mut menu_selected = 0; // 0 for "Start Game", 1 for "Quit"
    let mut game_running = false;

    // Show the start menu before the game begins
    while !game_running {
        terminal.draw(|frame| {
            render_start_menu(frame); // Render the start menu
        })?;

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Down => {
                        menu_selected = (menu_selected + 1) % 2; // Toggle between "Start Game" and "Quit"
                    }
                    KeyCode::Up => {
                        menu_selected = if menu_selected == 0 { 1 } else { 0 };
                    }
                    KeyCode::Enter => {
                        if menu_selected == 0 {
                            game_running = true; // Start the game
                        } else {
                            break; // Quit the game
                        }
                    }
                    KeyCode::Esc => {
                        break; // Quit on Esc key
                    }
                    _ => {}
                }
            }
        }
    }

    // Now start the game logic
    while game_running {
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(KeyEvent {
                code, modifiers: _, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Left => game.move_shape(-1, 0),
                    KeyCode::Right => game.move_shape(1, 0),
                    KeyCode::Down => game.move_shape(0, 1),
                    KeyCode::Up => game.rotate_shape(),
                    KeyCode::Char('p') => game.paused = !game.paused, // Toggle pause with the 'P' key
                    KeyCode::Esc => break,                            // Exit the game with Esc
                    _ => {}
                }
            }
        }

        // If the game is not paused, continue the game logic
        if !game.paused && game.last_tick.elapsed() > game.tick_rate {
            game.last_tick = Instant::now();
            game.move_shape(0, 1); // Move the shape down over time
        }

        if game.is_game_over() {
            // Show the Game Over screen
            terminal.draw(|frame| {
                render_gameover_menu(frame);
            })?;

            // Wait for user input to either restart or quit
            while let crossterm::event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('r') => {
                        game = Game::new(); // Restart the game
                        break; // Exit the game over loop
                    }
                    KeyCode::Char('q') => {
                        game_running = false; // Quit the game
                        break; // Exit the game over loop
                    }
                    _ => {}
                }
            }
        } else {
            render(&mut terminal, &game)?; // Render the game state (either paused or running)
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
