use crate::common::UserOption;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};

pub fn get_user_option(title: &str, options: Vec<UserOption>) -> char {
    let mut stdout = stdout();
    println!("{}\r", title);

    execute!(stdout, cursor::Hide).unwrap();

    enable_raw_mode().unwrap();
    let char;

    let mut selected_index = 0;
    let num_options = options.len();

    loop {
        // Move cursor to the beginning of the line and clear the line
        print!("\r");
        for (i, option) in options.iter().enumerate() {
            if i == selected_index {
                println!(
                    "{}{}\r",
                    "> ".with(Color::Green),
                    option.label.as_str().with(Color::Green)
                );
            } else {
                println!("{}{}\r", "  ", option.label);
            }
        }

        stdout.flush().unwrap();

        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read().unwrap()
        {
            match code {
                KeyCode::Up | KeyCode::Char('k') if selected_index > 0 => {
                    selected_index -= 1;
                }
                KeyCode::Down | KeyCode::Char('j') if selected_index < num_options - 1 => {
                    selected_index += 1;
                }
                KeyCode::Enter => {
                    char = options[selected_index].keybind;
                    break;
                }
                KeyCode::Char('n')
                    if modifiers.contains(KeyModifiers::CONTROL)
                        && selected_index < num_options - 1 =>
                {
                    selected_index += 1;
                }
                KeyCode::Char('p')
                    if modifiers.contains(KeyModifiers::CONTROL) && selected_index > 0 =>
                {
                    selected_index -= 1;
                }
                KeyCode::Esc => {
                    char = 'q';
                    break;
                }
                KeyCode::Char(c) => {
                    if let Some(option) = options.iter().find(|o| o.keybind == c) {
                        char = option.keybind;
                        break;
                    }
                }
                _ => {}
            }
        }

        // Move cursor up to redraw the menu in place
        for _ in 0..num_options {
            print!("\x1b[A\x1b[2K"); // Move cursor up and clear the line
        }
    }

    disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show).unwrap();

    // Move cursor up to redraw the menu in place
    for _ in 0..(num_options + 1) {
        print!("\x1b[A\x1b[2K"); // Move cursor up and clear the line
    }

    char
}
