use serde_derive::Deserialize;
use std::io::{stdin, stdout};
use std::io::{Result, Write};
use std::path::Path;
use std::process::Command;
use std::{fs, usize};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Deserialize, Debug)]
struct Config {
    choices: Choices,
}

#[derive(Deserialize, Debug)]
struct Choices {
    paths: Vec<String>,
}

fn index_to_char(i: usize) -> Option<char> {
    match i {
        0..=9 => ('0'..='9').nth(i),
        10.. => ('a'..='z').nth(i - 10),
        _ => None,
    }
}

fn try_map_to_index(c: char) -> Option<usize> {
    return match c {
        '0'..='9' => ('0'..='9').position(|x| x == c),
        'a'..='z' => ('a'..='z').position(|x| x == c).and_then(|i| Some(i + 10)),
        _ => None,
    };
}

fn try_read_choice_as_index() -> Option<usize> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout
        .write(b"Type choice: ")
        .and_then(|_| stdout.flush())
        .unwrap();

    let stdin = stdin();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char(c) => {
                let index = try_map_to_index(c);
                if index.is_some() {
                    return index;
                }
            }
            Key::Esc => break,
            _ => {}
        }

        stdout.flush().unwrap();
    }
    None
}

fn read_quick_edit_cfg() -> Config {
    let cfg_file = Path::new(option_env!("QE_CONFIG_FILE").unwrap_or(".quick_edit.toml"));
    let cfg_slice = fs::read(&cfg_file).expect(&format!(
        "Could not read config file '{}'!",
        cfg_file.display()
    ));

    toml::from_slice(&cfg_slice).expect(&format!(
        "Failed to parse config file '{}'!",
        cfg_file.display()
    ))
}

fn main() -> Result<()> {
    let config = read_quick_edit_cfg();
    for (index, path) in config.choices.paths.iter().enumerate() {
        let index_char = index_to_char(index).expect(&format!(
            "Cannot enumerate more than {index} elements! \
             Quick edit is intended for short selections, \
             which can be typed with a single keystroke."
        ));

        println!("{}): {}", index_char, path);
    }

    let choice_index = try_read_choice_as_index();
    if choice_index.is_none() {
        // No valid choice by user, return success:
        return Ok(());
    };

    let choice_path = config.choices.paths.get(choice_index.unwrap());
    Command::new("sh")
        .arg("-c")
        .arg(format!("$EDITOR {}", choice_path.unwrap()))
        .status()
        .expect("Failed to run editor command!");

    Ok(())
}
