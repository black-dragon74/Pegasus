//
//  Code is poetry
//
//  Created by Nick aka black.dragon74
//

// Imports go here
use reqwest;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Payload CMD structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Cmd {
    pub elevated: bool,
    pub process: String,
    pub args: Vec<String>,
}

/// The pyaload root structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub active: bool,
    pub user: String,
    pub password: String,
    pub commands: Vec<Cmd>,
}

/// Executes a shell command and returns wrapped STDOUT value
pub fn execute_command(command: String, args: Vec<String>) -> Option<String> {
    let mut command = Command::new(command);

    for arg in args {
        command.arg(arg);
    }

    let out = command.output();

    let msg = match out {
        Ok(msg) => Some(String::from_utf8_lossy(&msg.stdout).to_string()),
        Err(_) => None,
    };

    return msg;
}

/// Executes a shell command as root user and returns wrapped STDOUT value
pub fn execute_elevated_command(
    password: &String,
    command: String,
    args: Vec<String>,
) -> Option<String> {
    // Create a sh instance
    let mut executable = Command::new("sh");

    executable.arg("-c");

    let mut rest_commands = "echo '".to_owned();

    rest_commands.push_str(&password);
    rest_commands.push_str("' | sudo -S ");

    rest_commands.push_str(&command);
    rest_commands.push_str(" ");

    for arg in args {
        rest_commands.push_str(&arg);
        rest_commands.push_str(" ");
    }

    // Append the formatted command as last arg
    executable.arg(rest_commands);

    // Now that we have formatted the command, let's execute it and get the output
    let out = executable.output();

    // Parse the stdout
    let resp = match out {
        Ok(res) => Some(String::from_utf8_lossy(&res.stdout).to_string()),
        Err(_) => None,
    };

    // Return the stdout
    return resp;
}

/// Returns content of a webpage URL wrapped in a 'core::Result'
pub fn get_url_contents(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(url)?.text()?;

    Ok(body)
}

/// Checks if the internet connection is available and then returs a 'bool' for the same
pub fn internet_is_available() -> bool {
    return match reqwest::blocking::get("https://google.com") {
        Ok(_) => true,
        Err(_) => false,
    };
}

/// Shorthand wrapper around println macro. Used to print simple unformatted strings to STDOUT
pub fn say(msg: &str) {
    println!("{}", msg);
}
