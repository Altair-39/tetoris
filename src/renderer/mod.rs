use crate::game::control::{COLS, ROWS};
use crate::game::Game;
use ratatui::{
    layout::{Alignment, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::Result;

pub fn render_start_menu(frame: &mut ratatui::Frame) {
    let menu_items = ["Start Game", "Quit"];
    let mut menu_output = String::new();

    for (i, item) in menu_items.iter().enumerate() {
        menu_output.push_str(item);
        if i != menu_items.len() - 1 {
            menu_output.push('\n'); // Add newline between menu items
        }
    }

    let paragraph = Paragraph::new(menu_output)
        .block(
            Block::default()
                .title("TETORIS - Main Menu")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta))
                .style(Style::default().bg(Color::Black).fg(Color::White)),
        )
        .alignment(Alignment::Center) // Center the menu text
        .style(Style::default().fg(Color::Yellow));

    let size = frame.area();
    let area = Rect::new(
        size.width / 4,
        size.height / 4,
        size.width / 2,
        size.height / 2,
    );

    frame.render_widget(paragraph, area);
}

pub fn render_gameover_menu(frame: &mut ratatui::Frame) {
    let gameover_message = "Game Over\n\nPress 'R' to Restart or 'Q' to Quit";

    let paragraph = Paragraph::new(gameover_message)
        .block(
            Block::default()
                .title("GAME OVER")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .style(Style::default().bg(Color::Black).fg(Color::White)),
        )
        .alignment(Alignment::Center) // Center the game over message
        .style(Style::default().fg(Color::Yellow));

    let size = frame.area();
    let area = Rect::new(
        size.width / 4,
        size.height / 4,
        size.width / 2,
        size.height / 2,
    );

    frame.render_widget(paragraph, area);
}

pub fn render(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    game: &Game,
) -> Result<()> {
    terminal
        .draw(|frame| {
            if game.paused {
                render_pause_menu(frame);
            } else {
                // Render the main game area
                render_game_area(frame, game);

                // Render the next shape preview
                render_next_shape_preview(frame, game);

                // Render the current falling shape with its unique color
                render_falling_shape(frame, game);
            }
        })
        .map(|_| ())
}

pub fn render_pause_menu(frame: &mut Frame) {
    // Displaying a static message as the pause prompt
    let pause_message = "Game Paused\n\nPress 'P' to resume";

    // Example ASCII Art for the pause menu
    let ascii_art = r#"
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠛⢛⡛⠯⣿⣿⣿⣿⣿⣿⣿⣿⡿⢋⣵⡾⣸⣹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⣴⢣⣾⣿⣿⣿⣦⡙⣿⣿⣿⠁⣿⡟⣴⣿⣿⡇⣿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠹⡘⣿⣿⣿⣿⠿⢿⣿⣿⢇⣿⣌⠸⣿⣿⣿⣷⠹⣧⡏⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⢛⡡⠘⠀⠒⠀⠀⠿⣤⡔⣬⡈⢿⢿⣧⢿⡿⢿⡟⣰⡜⢱⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⢛⠛⣛⣛⣛⠿⢋⠔⣡⣶⡆⢃⣀⣂⠽⣃⣦⣅⠚⠻⢈⠢⡁⠬⢁⣁⣈⠩⣍⠹⠛⣛⣛⠿⢿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢋⠵⣾⡁⠀⠨⠭⣤⠌⡄⣠⡆⣿⠟⣴⣿⣷⡭⡾⠟⣫⢁⢻⣦⠀⣄⠠⠐⠍⠛⢉⣁⣀⣛⣃⣄⣈⣽⣶⡍⠛⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡗⢸⠿⡐⢤⣭⣭⣤⠆⠀⡴⢋⣶⡍⣼⠿⠿⠿⠷⠷⠛⠁⣤⣿⠟⣢⡜⠀⠀⣲⣭⣭⣭⠙⢛⣋⣉⠭⠵⣋⡔⣰⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢈⠑⠢⠍⣛⠻⣿⠀⣾⣇⠺⣿⠀⢃⢟⠲⣰⣲⢄⡟⣙⣋⣴⣶⣌⢿⡌⠀⡿⣛⡩⠴⠾⢋⣁⣤⣶⣿⠿⢁⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⢹⡇⢰⣶⣾⣿⢸⣹⣿⣷⣶⠀⣼⢬⡯⣦⣿⡄⢇⢼⣿⣿⡈⣿⣆⢻⡌⠸⢷⣦⣬⣭⣭⣭⡭⢀⠶⢂⣽⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠖⣀⠀⠹⣿⡏⢸⣟⣿⣿⡏⡄⢿⠺⢹⣼⣿⡇⢀⠠⠹⣿⣷⡘⣿⣆⠨⡀⢀⢠⠬⠭⢉⠡⠐⣁⣤⠹⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠩⢩⣵⣶⡆⢸⣿⠋⡿⢐⡓⠈⢇⠀⢿⣿⣿⢈⢁⣤⡈⠋⠳⡹⣿⡆⠃⢄⣐⣤⣭⣤⣍⢿⠟⡃⢀⣾⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⢢⡤⢬⣼⡇⠘⡾⢸⠃⠋⠉⠁⠈⠂⠀⠻⣿⡌⠏⠀⠀⠀⠀⠀⠛⠿⡄⢼⣿⣟⠭⣉⣀⣒⣛⢛⣼⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡗⠈⠙⣛⠃⡰⠁⣡⢀⠰⠀⠀⠀⣠⣀⠢⢈⠓⠸⡐⠀⢀⠄⢠⢲⠀⠶⠀⠈⠉⠈⠉⠉⣁⠐⣾⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡌⠳⢶⣂⣠⠄⢿⠀⢧⣅⣠⣤⣽⣿⣿⣷⣮⣿⣷⣴⣦⡾⢀⡏⠨⣀⠘⠿⣦⡐⠚⢩⡥⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡌⢀⢀⡀⢲⡄⠢⠘⣿⣿⣿⣿⣿⡿⡿⣿⣿⣿⣿⣏⡀⠸⢠⣾⣿⠆⢁⡀⠉⠉⠁⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣌⠋⣠⣾⣿⣆⠀⢜⠿⣿⣿⡟⠻⠿⠟⢻⣿⣿⠟⠁⣠⣾⣿⣧⡂⠄⠀⠊⠁⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⣿⣿⣿⣿⣿⣷⣶⣿⣿⣿⣷⣦⣷⡬⠛⠿⣷⣶⣾⠿⣋⣵⢠⡀⠻⠿⣿⣿⣿⡟⠆⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⢰⣿⣶⣾⣭⣽⣿⡶⠭⠉⣩⠎⠉⠛⠛⠻⢡⣄⡀⠈⠐⠛⠋⢉⣀⡀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠇⢸⣿⣿⣿⣿⣽⣥⠖⢠⣿⣿⡆⡀⣶⡀⢶⣮⣿⣿⠀⢼⣉⣉⣀⡿⠗⠘⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡿⠩⠍⡉⠶⠀⣠⣬⠈⠉⠉⠈⣴⣶⣿⣶⣮⣭⣥⣁⠛⠉⢢⡙⠿⣿⢤⠀⠀⠉⠈⠀⠾⢧⣄⡂⠐⠉⢉⣙⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⠟⠛⣛⣛⣉⠁⠀⠀⢿⣿⡇⢄⡀⢿⣿⡆⠀⠀⠀⣟⣏⣿⣿⣿⣿⣿⣿⣦⣄⠌⣛⠒⡶⠁⠀⠀⢘⠛⣷⡆⠀⠋⠕⣂⣴⣯⣄⠈⠀⠙⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⡁⠀⠀⠈⠻⣿⠀⠀⠀⢸⠟⠧⠀⠁⢈⠀⠁⠀⠀⠀⣥⢠⠠⣿⣿⣿⣿⡻⣬⣄⠉⠒⠬⣄⣀⠀⠼⣤⣤⠟⠁⠀⣠⣾⣿⣿⣿⡟⢰⢸⡄⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣦⡀⠀⠀⠀⠁⠀⠀⠈⠄⠀⠀⠀⠀⠈⠀⢀⣀⣠⠇⠆⠋⠀⠻⢿⣿⠛⣧⠨⣧⡀⠀⣀⣴⣶⡶⢟⡅⠀⠀⠸⣿⣿⣿⣿⡿⠀⣸⣾⣷⠐⡘⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣷⠦⣤⣀⡀⠀⠀⠀⢀⣤⡀⢶⠌⢧⡘⠻⣿⠀⠀⠀⢀⣴⡌⣿⠀⠘⣦⢨⡙⢦⣙⠟⠋⠒⣫⣴⢇⠀⠀⠘⠿⣿⡿⢡⡇⣿⣿⣿⡆⢱⡘⣿⣿⣿⣿⣿⣿⣿⣿
⣿⡿⢋⡴⠿⢛⡩⠀⠀⠀⠠⠾⠿⠿⠆⠠⣢⡍⠐⠈⢰⣶⣿⣿⣿⣧⢹⢰⡀⠸⡇⠿⠃⠈⢂⡀⠖⣸⡟⠈⣦⡀⠀⠀⠀⠀⠘⢷⣿⣿⡇⠇⠀⠑⠘⣿⣿⣿⣿⣿⣿⣿
⣛⣉⣥⣤⣭⣴⣶⣶⣾⣿⣿⣿⣿⣿⣿⣶⣬⣭⣤⡀⣾⣿⣿⣿⡿⠿⡌⠈⡧⠀⠉⠀⣠⣶⣿⣿⣿⣿⠇⢱⣿⣿⣶⡤⠀⠀⠀⠀⠉⠙⠋⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⢻⣿⣿⣿⢃⣿⣿⣿⣿⣇⡀⠘⠀⠀⣀⣤⣼⣿⣿⣿⣿⣿⡟⠀⣾⣿⡿⠛⡴⢠⣷⣄⡀⠐⠐⠀⠀⢀⠈⢶⣦⠘⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⢻⣿⡿⢸⣿⣿⡿⠛⠋⠀⢀⣤⣾⣿⣿⣿⣧⡏⠀⠈⣿⢃⢰⣿⢏⣴⣾⠃⣾⡿⣻⣿⣿⡇⣿⣷⡘⠂⢨⣭⣴⠘⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⢧⠻⠇⢿⠟⠁⠀⢀⣀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣶⣿⡟⡀⢞⣴⣿⣿⡟⣰⣦⢥⣭⣭⣭⣥⣭⣭⣤⠘⣈⣩⣥⣴⣿⠟⢿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⢛⡤⡞⢈⠀⢀⣤⣶⠉⠉⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢃⢠⣿⣿⣿⡿⠁⣍⣙⣛⣛⠛⣛⡋⠉⠉⠛⢠⢸⣿⣿⣿⣿⣷⣮⣻
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣵⣾⣿⡇⡀⣤⣶⣾⣿⣿⣶⣶⣿⣿⣿⣿⣿⣿⣿⠋⠉⢻⡟⡀⣾⣿⣿⣿⢃⢠⢻⣿⣿⣿⡆⠛⠀⠀⠐⣱⣿⡆⢿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢳⣧⡉⢩⣭⣍⢛⡛⠻⠿⢿⣿⣿⣿⣿⣿⣷⣤⣾⠁⢰⣿⣿⣿⣏⡆⣼⣷⣝⢿⣿⣿⡄⠀⠀⣼⣿⣿⣧⠘⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣼⣿⣿⣷⣦⢨⣬⣭⠭⣩⣤⣤⢈⣉⢙⡛⠻⠿⠏⠄⣼⣿⣿⣿⡿⢠⣿⣿⣿⣿⣿⣿⣷⡀⣸⣿⣿⣿⣿⢣⢻⣿⣿⣿⣿⣿    "#;

    let paragraph = Paragraph::new(format!("{}\n\n{}", pause_message, ascii_art))
        .block(
            Block::default()
                .title("PAUSED")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta))
                .style(Style::default().bg(Color::Black).fg(Color::White)),
        )
        .alignment(Alignment::Center) // Center the text in the block
        .style(Style::default().fg(Color::Yellow));

    // Get the terminal size and define the area for the pause menu
    let size = frame.area();
    let area = Rect::new(
        size.width / 4,
        size.height / 4,
        size.width / 2,
        size.height / 2,
    );

    // Render the paragraph with centered text and a bordered block
    frame.render_widget(paragraph, area);
}
/// Render the main game area (game board)

fn precompute_active_cells(game: &Game) -> Vec<Vec<bool>> {
    let mut active_cells = vec![vec![false; COLS]; ROWS];

    // Mark the active cells of the shape on the game board
    for x in 0..game.current.width {
        for y in 0..game.current.width {
            if game.current.array[x][y] == 1 {
                let shape_row = game.current.row + x as isize;
                let shape_col = game.current.col + y as isize;

                if shape_row >= 0
                    && shape_col >= 0
                    && shape_row < ROWS as isize
                    && shape_col < COLS as isize
                {
                    active_cells[shape_row as usize][shape_col as usize] = true;
                }
            }
        }
    }

    active_cells
}

fn render_game_area(frame: &mut ratatui::Frame, game: &Game) {
    let active_cells = precompute_active_cells(game);

    let mut output = Vec::with_capacity((20 * 11 * 2) + 20); // Preallocate memory

    for i in 0..20 {
        for j in 0..11 {
            let mut cell = b'.'; // Default to empty space

            // Check if the cell is occupied by the active shape or the game board
            if game.table[i][j] == 1 || active_cells[i][j] {
                cell = b'O'; // Active cell in the game or active shape cell
            }

            output.push(cell);
            output.push(b' '); // Space for readability
        }
        output.push(b'\n');
    }

    // Append score using efficient formatting
    output.extend_from_slice(format!("\nScore: {}\n", game.score).as_bytes());

    // Convert Vec<u8> to String once
    let output_str = String::from_utf8(output).unwrap();

    let paragraph = Paragraph::new(output_str)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, frame.area());
}

/// Render the preview of the next shape

fn render_next_shape_preview(frame: &mut ratatui::Frame, game: &Game) {
    let mut output = Vec::with_capacity(game.next_shape.width * game.next_shape.width * 2 + 15);

    output.extend_from_slice(b"\nNext Shape:\n");

    for i in 0..game.next_shape.width {
        for j in 0..game.next_shape.width {
            let cell = if game.next_shape.array[i][j] == 1 {
                b'O'
            } else {
                b'.'
            };
            output.push(cell);
            output.push(b' '); // Add space for spacing
        }
        output.push(b'\n');
    }

    let preview_output = String::from_utf8(output).unwrap(); // Convert once

    let preview_paragraph = Paragraph::new(preview_output)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    let preview_area = ratatui::layout::Rect::new(
        frame.area().width.saturating_sub(20), // Avoid negative overflow
        0,
        20,
        7,
    );

    frame.render_widget(preview_paragraph, preview_area);
}

/// Render the current falling shape with its unique color
fn render_falling_shape(frame: &mut ratatui::Frame, game: &Game) {
    let mut output = Vec::with_capacity(20 * 11 * 2);

    let mut shape_cells = vec![];
    for x in 0..game.current.width {
        for y in 0..game.current.width {
            if game.current.array[x][y] == 1 {
                let shape_row = (game.current.row + x as isize) as usize;
                let shape_col = (game.current.col + y as isize) as usize;
                if shape_row < 20 && shape_col < 11 {
                    shape_cells.push((shape_row, shape_col));
                }
            }
        }
    }

    for i in 0..20 {
        for j in 0..11 {
            let mut cell = if game.table[i][j] == 1 { b'O' } else { b'.' };
            if shape_cells.contains(&(i, j)) {
                cell = b'O';
            }
            output.push(cell);
            output.push(b' ');
        }
        output.push(b'\n');
    }

    let output_with_color = String::from_utf8(output).unwrap();

    let game_area_paragraph = Paragraph::new(output_with_color)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(game.current.color.into()));

    frame.render_widget(game_area_paragraph, frame.area());
}
