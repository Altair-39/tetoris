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
fn render_game_area(frame: &mut ratatui::Frame, game: &Game) {
    let mut output = String::new();

    for i in 0..20 {
        for j in 0..11 {
            let mut cell = if game.table[i][j] == 1 { "O" } else { "." };
            for x in 0..game.current.width {
                for y in 0..game.current.width {
                    let shape_row = game.current.row + x as isize;
                    let shape_col = game.current.col + y as isize;

                    if shape_row >= 0
                        && shape_col >= 0
                        && shape_row as usize == i
                        && shape_col as usize == j
                        && game.current.array[x][y] == 1
                    {
                        cell = "O";
                    }
                }
            }
            output.push_str(cell);
            output.push(' ');
        }
        output.push('\n');
    }

    output.push_str(&format!("\nScore: {}\n", game.score));

    let paragraph = Paragraph::new(output)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, frame.area());
}

/// Render the preview of the next shape
fn render_next_shape_preview(frame: &mut ratatui::Frame, game: &Game) {
    let mut preview_output = String::new();
    preview_output.push_str("\nNext Shape:\n");
    for i in 0..game.next_shape.width {
        for j in 0..game.next_shape.width {
            let cell = if game.next_shape.array[i][j] == 1 {
                "O"
            } else {
                "."
            };
            preview_output.push_str(cell);
            preview_output.push(' ');
        }
        preview_output.push('\n');
    }

    let preview_paragraph = Paragraph::new(preview_output)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    let preview_area = ratatui::layout::Rect::new(
        frame.area().width - 20, // x-position
        0,                       // y-position
        20,                      // width
        7,                       // height (enough to show the next shape preview)
    );

    frame.render_widget(preview_paragraph, preview_area);
}

/// Render the current falling shape with its unique color
fn render_falling_shape(frame: &mut ratatui::Frame, game: &Game) {
    let mut output_with_color = String::new();
    for i in 0..20 {
        for j in 0..11 {
            let mut cell = if game.table[i][j] == 1 { "O" } else { "." };
            for x in 0..game.current.width {
                for y in 0..game.current.width {
                    let shape_row = game.current.row + x as isize;
                    let shape_col = game.current.col + y as isize;

                    if shape_row >= 0
                        && shape_col >= 0
                        && shape_row as usize == i
                        && shape_col as usize == j
                        && game.current.array[x][y] == 1
                    {
                        cell = "O";
                    }
                }
            }
            output_with_color.push_str(cell);
            output_with_color.push(' ');
        }
        output_with_color.push('\n');
    }

    let game_area_paragraph = Paragraph::new(output_with_color)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(game.current.color.into())); // Use the shape's unique color

    frame.render_widget(game_area_paragraph, frame.area());
}
